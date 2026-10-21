use cc_config::RuntimeConfig;
use cc_orchestrator::{RolloutOrchestrator, RolloutReport, RolloutRequest};
use std::path::Path;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("rejected: {0}")]
    Rejected(String),
}
pub struct RolloutApi(RolloutOrchestrator);
impl Default for RolloutApi {
    fn default() -> Self {
        Self(RolloutOrchestrator::new(RuntimeConfig::default()))
    }
}
impl RolloutApi {
    pub async fn evaluate(&self, r: RolloutRequest, p: &Path) -> Result<RolloutReport, ApiError> {
        self.0
            .run(r, p)
            .await
            .map_err(|e| ApiError::Rejected(e.to_string()))
    }
}
