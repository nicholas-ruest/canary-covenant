use cc_types::{CohortId, ReleaseId};
use open_feature::{
    provider::{FeatureProvider, ProviderMetadata, ProviderStatus, ResolutionDetails},
    EvaluationContext, EvaluationError, EvaluationResult, OpenFeature, StructValue,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone)]
struct Provider {
    meta: ProviderMetadata,
    enabled: bool,
}
impl Provider {
    fn new(enabled: bool) -> Self {
        Self {
            meta: ProviderMetadata::new("canary-covenant-local"),
            enabled,
        }
    }
}
#[open_feature::async_trait]
impl FeatureProvider for Provider {
    fn metadata(&self) -> &ProviderMetadata {
        &self.meta
    }
    fn status(&self) -> ProviderStatus {
        ProviderStatus::Ready
    }
    async fn resolve_bool_value(
        &self,
        k: &str,
        _: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<bool>> {
        if k == "canary-release-enabled" {
            Ok(ResolutionDetails::new(self.enabled))
        } else {
            Err(EvaluationError {
                code: open_feature::EvaluationErrorCode::FlagNotFound,
                message: Some("unknown flag".into()),
            })
        }
    }
    async fn resolve_int_value(
        &self,
        _: &str,
        _: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<i64>> {
        Ok(ResolutionDetails::new(0))
    }
    async fn resolve_float_value(
        &self,
        _: &str,
        _: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<f64>> {
        Ok(ResolutionDetails::new(0.))
    }
    async fn resolve_string_value(
        &self,
        _: &str,
        _: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<String>> {
        Ok(ResolutionDetails::new(String::new()))
    }
    async fn resolve_struct_value(
        &self,
        _: &str,
        _: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<StructValue>> {
        Err(EvaluationError {
            code: open_feature::EvaluationErrorCode::TypeMismatch,
            message: Some("unsupported".into()),
        })
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlagDecision {
    pub eligible: bool,
    pub provider: String,
    pub cohort: String,
}
#[derive(Debug, Error)]
pub enum FlagError {
    #[error("evaluation failed: {0}")]
    Evaluation(String),
}
pub struct FlagGate {
    enabled: bool,
}
impl FlagGate {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    pub async fn evaluate(&self, r: &ReleaseId, c: &CohortId) -> Result<FlagDecision, FlagError> {
        let mut api = OpenFeature::singleton_mut().await;
        api.set_provider(Provider::new(self.enabled)).await;
        drop(api);
        let client = OpenFeature::singleton().await.create_client();
        let ctx = EvaluationContext::default()
            .with_targeting_key(r.as_str())
            .with_custom_field("cohort", c.as_str().to_string());
        let d = client
            .get_bool_details("canary-release-enabled", Some(&ctx), None)
            .await
            .map_err(|e| FlagError::Evaluation(format!("{e:?}")))?;
        Ok(FlagDecision {
            eligible: d.value,
            provider: "canary-covenant-local".into(),
            cohort: c.as_str().into(),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn official_sdk_executes() {
        let d = FlagGate::new(true)
            .evaluate(
                &ReleaseId::new("r").unwrap(),
                &CohortId::new("beta").unwrap(),
            )
            .await
            .unwrap();
        assert!(d.eligible)
    }
}
