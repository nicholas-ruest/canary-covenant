use cc_config::RuntimeConfig;
use cc_metrics::{Observation, ObservationBatch};
use cc_orchestrator::{RolloutOrchestrator, RolloutReport, RolloutRequest, Variant};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub variant: Variant,
    pub fixture_hash: String,
    pub report: RolloutReport,
}
pub struct Evaluator;
impl Evaluator {
    pub fn fixture(h: bool) -> ObservationBatch {
        ObservationBatch::new(
            (1..=30)
                .map(|i| Observation::new(i, h, if h { 900. } else { 80. }, !h).unwrap())
                .collect(),
        )
        .unwrap()
    }
    pub async fn run(v: Variant, h: bool) -> Result<EvaluationResult, String> {
        let f = Self::fixture(h);
        let hash = hex::encode(Sha256::digest(serde_json::to_vec(&f).unwrap()));
        let d = tempfile::tempdir().map_err(|e| e.to_string())?;
        let report = RolloutOrchestrator::new(RuntimeConfig::default())
            .run(
                RolloutRequest {
                    release_id: format!("eval-{v:?}-{h}"),
                    version: "2.7".into(),
                    digest: "a".repeat(64),
                    observations: f,
                    variant: v,
                },
                d.path(),
            )
            .await
            .map_err(|e| e.to_string())?;
        Ok(EvaluationResult {
            variant: v,
            fixture_hash: hash,
            report,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn reproducible() {
        assert_eq!(
            Evaluator::run(Variant::Combined, false)
                .await
                .unwrap()
                .fixture_hash,
            Evaluator::run(Variant::Combined, false)
                .await
                .unwrap()
                .fixture_hash
        )
    }
}
