use cc_incidents::Severity;
use cc_metrics::ObservationBatch;
use cc_ruvector::IncidentMatch;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub harm_rate: f64,
    pub e_value: f64,
    pub incident_penalty: f64,
    pub budget_remaining: f64,
    pub reasons: Vec<String>,
}
pub struct RiskEngine;
impl RiskEngine {
    pub fn assess(b: &ObservationBatch, e: f64, m: &[IncidentMatch]) -> RiskAssessment {
        let h = b.harm_rate();
        let p = m
            .iter()
            .map(|x| match x.severity {
                Severity::Low => 0.05,
                Severity::Medium => 0.15,
                Severity::High => 0.35,
                Severity::Critical => 1.,
            })
            .fold(0., f64::max);
        RiskAssessment {
            harm_rate: h,
            e_value: e,
            incident_penalty: p,
            budget_remaining: (0.2 - h).max(0.),
            reasons: vec![
                format!("harm_rate={h:0.3}"),
                format!("e_value={e:0.3}"),
                format!("incident_penalty={p:0.2}"),
            ],
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cc_metrics::Observation;
    #[test]
    fn penalty() {
        let b = ObservationBatch::new(vec![Observation::new(1, false, 1., true).unwrap()]).unwrap();
        assert_eq!(
            RiskEngine::assess(
                &b,
                2.,
                &[IncidentMatch {
                    id: "x".into(),
                    severity: Severity::High,
                    score: 1.
                }]
            )
            .incident_penalty,
            0.35
        )
    }
}
