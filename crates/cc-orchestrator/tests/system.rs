use cc_config::RuntimeConfig;
use cc_metrics::{Observation, ObservationBatch};
use cc_orchestrator::{RolloutOrchestrator, RolloutRequest, Variant};
use cc_types::Decision;

fn observations(harmful: bool) -> ObservationBatch {
    ObservationBatch::new(
        (1..=30)
            .map(|sequence| {
                Observation::new(
                    sequence,
                    harmful,
                    if harmful { 900.0 } else { 80.0 },
                    !harmful,
                )
                .expect("fixture is valid")
            })
            .collect(),
    )
    .expect("batch is non-empty and ordered")
}

async fn execute(harmful: bool) -> cc_orchestrator::RolloutReport {
    let work = tempfile::tempdir().expect("temporary work directory");
    RolloutOrchestrator::new(RuntimeConfig::default())
        .run(
            RolloutRequest {
                release_id: format!("integration-{harmful}"),
                version: "2.7.0".into(),
                digest: "a".repeat(64),
                observations: observations(harmful),
                variant: Variant::Combined,
            },
            work.path(),
        )
        .await
        .expect("combined system executes")
}

#[tokio::test]
async fn healthy_promotes_with_all_integrations_and_verified_witness() {
    let report = execute(false).await;
    assert_eq!(report.decision, Decision::Promote);
    assert_eq!(
        report.integrations,
        ["openfeature", "ruvector", "rvf", "eprocess"]
    );
    assert!(report.grant_consumed);
    assert!(report.verified);
    assert!(!report.recalled_incidents.is_empty());
}

#[tokio::test]
async fn harmful_rolls_back_and_executes_recovery() {
    let report = execute(true).await;
    assert_eq!(report.decision, Decision::Rollback);
    assert_eq!(report.recovery_actions, 2);
    assert!(report.verified);
    assert!(!report.grant_consumed);
}
