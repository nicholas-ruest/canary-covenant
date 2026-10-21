use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Incident {
    pub id: String,
    pub component: String,
    pub severity: Severity,
    pub features: [f32; 4],
    pub mitigation: String,
}
#[derive(Debug, Error)]
pub enum IncidentError {
    #[error("empty field")]
    Empty,
    #[error("invalid features")]
    Features,
}
impl Incident {
    pub fn new(
        id: &str,
        component: &str,
        severity: Severity,
        features: [f32; 4],
        mitigation: &str,
    ) -> Result<Self, IncidentError> {
        if [id, component, mitigation]
            .iter()
            .any(|x| x.trim().is_empty())
        {
            return Err(IncidentError::Empty);
        }
        if features
            .iter()
            .any(|x| !x.is_finite() || !(0.0..=1.0).contains(x))
        {
            return Err(IncidentError::Features);
        }
        Ok(Self {
            id: id.into(),
            component: component.into(),
            severity,
            features,
            mitigation: mitigation.into(),
        })
    }
}
pub fn seeded() -> Vec<Incident> {
    vec![
        Incident::new(
            "latency-tail",
            "router",
            Severity::Low,
            [0.02, 0.15, 0.01, 0.],
            "reduce cohort",
        )
        .unwrap(),
        Incident::new(
            "tool-timeouts",
            "tools",
            Severity::High,
            [0.25, 0.55, 0.75, 0.1],
            "rollback tools",
        )
        .unwrap(),
        Incident::new(
            "policy-bypass",
            "policy",
            Severity::Critical,
            [0.45, 0.35, 0.25, 1.],
            "revoke release",
        )
        .unwrap(),
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn seed_valid() {
        assert_eq!(seeded().len(), 3);
        assert!(Incident::new("", "x", Severity::Low, [0.; 4], "m").is_err())
    }
}
