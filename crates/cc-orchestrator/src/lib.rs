use cc_cohort::standard;
use cc_config::RuntimeConfig;
use cc_embedding::IncidentEmbedder;
use cc_eprocess::EProcess;
use cc_exposure::{ExposureLedger, ExposureRecord};
use cc_grant::RolloutGrant;
use cc_incidents::seeded;
use cc_ledger::AuditLedger;
use cc_metrics::ObservationBatch;
use cc_openfeature::FlagGate;
use cc_planner::Planner;
use cc_policy::{PolicyEngine, PolicyInput};
use cc_recovery::{RecoveryExecutor, RecoveryPlan};
use cc_release::ReleaseCandidate;
use cc_risk::RiskEngine;
use cc_ruvector::IncidentMemory;
use cc_rvf::{WitnessArtifact, WitnessChain};
use cc_telemetry::Telemetry;
use cc_types::{Decision, ReleaseId, SubjectId};
use cc_verifier::Verifier;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};
use thiserror::Error;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Variant {
    Combined,
    NoOpenFeature,
    NoRuVector,
    NoRvf,
    NoEProcess,
    EnterpriseOnly,
    RuvnetOnly,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolloutRequest {
    pub release_id: String,
    pub version: String,
    pub digest: String,
    pub observations: ObservationBatch,
    pub variant: Variant,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolloutReport {
    pub release_id: String,
    pub decision: Decision,
    pub reasons: Vec<String>,
    pub samples: usize,
    pub e_value: f64,
    pub recalled_incidents: Vec<String>,
    pub integrations: Vec<String>,
    pub exposure_count: usize,
    pub grant_consumed: bool,
    pub recovery_actions: usize,
    pub witness: Option<WitnessArtifact>,
    pub verified: bool,
}
#[derive(Debug, Error)]
pub enum OrchestratorError {
    #[error("domain: {0}")]
    Domain(String),
    #[error("integration: {0}")]
    Integration(String),
}
pub struct RolloutOrchestrator {
    config: RuntimeConfig,
}
impl RolloutOrchestrator {
    pub fn new(config: RuntimeConfig) -> Self {
        Self { config }
    }
    pub async fn run(
        &self,
        req: RolloutRequest,
        work: &Path,
    ) -> Result<RolloutReport, OrchestratorError> {
        let id = ReleaseId::new(req.release_id.clone())
            .map_err(|e| OrchestratorError::Domain(e.to_string()))?;
        let mut release = ReleaseCandidate::new(id.clone(), req.version, req.digest)
            .map_err(|e| OrchestratorError::Domain(e.to_string()))?;
        release
            .validate()
            .and_then(|_| release.activate())
            .map_err(|e| OrchestratorError::Domain(e.to_string()))?;
        let cohort = standard().current().clone();
        let of = matches!(
            req.variant,
            Variant::Combined
                | Variant::NoRuVector
                | Variant::NoRvf
                | Variant::NoEProcess
                | Variant::EnterpriseOnly
        );
        let rv = matches!(
            req.variant,
            Variant::Combined
                | Variant::NoOpenFeature
                | Variant::NoRvf
                | Variant::NoEProcess
                | Variant::RuvnetOnly
        );
        let wf = matches!(
            req.variant,
            Variant::Combined
                | Variant::NoOpenFeature
                | Variant::NoRuVector
                | Variant::NoEProcess
                | Variant::RuvnetOnly
        );
        let epok = matches!(
            req.variant,
            Variant::Combined | Variant::NoOpenFeature | Variant::NoRuVector | Variant::NoRvf
        );
        let flag = if of {
            FlagGate::new(true)
                .evaluate(&id, &cohort.id)
                .await
                .map_err(|e| OrchestratorError::Integration(e.to_string()))?
                .eligible
        } else {
            false
        };
        let mut exposure = ExposureLedger::default();
        for o in &req.observations.0 {
            exposure.record(ExposureRecord {
                release: id.clone(),
                cohort: cohort.id.clone(),
                subject: SubjectId::new(format!("subject-{}", o.sequence)).unwrap(),
            });
        }
        let mut matches = vec![];
        if rv {
            std::fs::create_dir_all(work)
                .map_err(|e| OrchestratorError::Integration(e.to_string()))?;
            let mut mem =
                IncidentMemory::open(&work.join(format!("{}-incidents.db", req.release_id)))
                    .map_err(|e| OrchestratorError::Integration(e.to_string()))?;
            mem.seed(&seeded())
                .map_err(|e| OrchestratorError::Integration(e.to_string()))?;
            matches = mem
                .recall(IncidentEmbedder::batch(&req.observations), 2)
                .map_err(|e| OrchestratorError::Integration(e.to_string()))?
        }
        let mut ep = EProcess::new(
            self.config.unacceptable_harm,
            self.config.alpha,
            self.config.bet,
        )
        .map_err(|e| OrchestratorError::Domain(e.to_string()))?;
        if epok {
            for o in &req.observations.0 {
                ep.observe(o.harm)
                    .map_err(|e| OrchestratorError::Domain(e.to_string()))?;
            }
        }
        let risk = RiskEngine::assess(&req.observations, ep.wealth(), &matches);
        let proposal = Planner::propose(
            &risk,
            req.observations.len(),
            self.config.min_samples,
            epok && ep.reached(),
        );
        let policy = PolicyEngine::decide(PolicyInput {
            flag,
            rvf: wf,
            ruvector: rv,
            eprocess: epok,
            proposal,
            risk,
        });
        let mut ledger = AuditLedger::default();
        ledger.append(id.clone(), "release-activated", &release.digest);
        ledger.append(id.clone(), "flag-evaluated", &flag.to_string());
        ledger.append(id.clone(), "decision", &format!("{:?}", policy.decision));
        let mut grant_consumed = false;
        let mut recovery_actions = 0;
        if policy.decision == Decision::Promote {
            let mut g = RolloutGrant::issue(
                id.clone(),
                cohort.id.clone(),
                cohort.ceiling,
                ledger.events().last().unwrap().hash.clone(),
            )
            .map_err(|e| OrchestratorError::Domain(e.to_string()))?;
            g.consume()
                .map_err(|e| OrchestratorError::Domain(e.to_string()))?;
            grant_consumed = true;
            ledger.append(id.clone(), "grant-consumed", &g.id);
        } else if policy.decision == Decision::Rollback {
            let p = RecoveryPlan::new(id.clone(), "0".repeat(64))
                .map_err(|e| OrchestratorError::Domain(e.to_string()))?;
            recovery_actions = RecoveryExecutor::default().execute(&p).len();
            ledger.append(
                id.clone(),
                "recovery-completed",
                &recovery_actions.to_string(),
            );
        }
        release
            .finish(policy.decision)
            .map_err(|e| OrchestratorError::Domain(e.to_string()))?;
        let witness = if wf {
            Some(WitnessChain::seal(&ledger.actions()))
        } else {
            None
        };
        let mut integrations = vec![];
        if of {
            integrations.push("openfeature".into())
        }
        if rv {
            integrations.push("ruvector".into())
        }
        if wf {
            integrations.push("rvf".into())
        }
        if epok {
            integrations.push("eprocess".into())
        }
        let verified = Verifier::verify(policy.decision, &ledger, witness.as_ref(), &integrations)
            .map(|x| x.valid)
            .unwrap_or(false);
        let mut t = Telemetry::default();
        t.emit(
            "rollout.decision",
            id.clone(),
            HashMap::from([("decision".into(), format!("{:?}", policy.decision))]),
        );
        Ok(RolloutReport {
            release_id: req.release_id,
            decision: policy.decision,
            reasons: policy.reasons,
            samples: req.observations.len(),
            e_value: ep.wealth(),
            recalled_incidents: matches.into_iter().map(|x| x.id).collect(),
            integrations,
            exposure_count: exposure.total(),
            grant_consumed,
            recovery_actions,
            witness,
            verified,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cc_metrics::Observation;
    fn b(h: bool) -> ObservationBatch {
        ObservationBatch::new(
            (1..=30)
                .map(|i| Observation::new(i, h, if h { 900. } else { 80. }, !h).unwrap())
                .collect(),
        )
        .unwrap()
    }
    fn r(h: bool, v: Variant) -> RolloutRequest {
        RolloutRequest {
            release_id: format!("r-{h}-{v:?}"),
            version: "2.7".into(),
            digest: "a".repeat(64),
            observations: b(h),
            variant: v,
        }
    }
    #[tokio::test]
    async fn combined_paths() {
        let d = tempfile::tempdir().unwrap();
        let a = RolloutOrchestrator::new(RuntimeConfig::default())
            .run(r(false, Variant::Combined), d.path())
            .await
            .unwrap();
        assert_eq!(a.decision, Decision::Promote);
        assert!(a.grant_consumed && a.verified);
        let d = tempfile::tempdir().unwrap();
        let z = RolloutOrchestrator::new(RuntimeConfig::default())
            .run(r(true, Variant::Combined), d.path())
            .await
            .unwrap();
        assert_eq!(z.decision, Decision::Rollback);
        assert!(z.recovery_actions > 0)
    }
    #[tokio::test]
    async fn ablations_hold() {
        for v in [
            Variant::NoOpenFeature,
            Variant::NoRuVector,
            Variant::NoRvf,
            Variant::NoEProcess,
            Variant::EnterpriseOnly,
            Variant::RuvnetOnly,
        ] {
            let d = tempfile::tempdir().unwrap();
            let x = RolloutOrchestrator::new(RuntimeConfig::default())
                .run(r(false, v), d.path())
                .await
                .unwrap();
            assert_ne!(x.decision, Decision::Promote)
        }
    }
}
