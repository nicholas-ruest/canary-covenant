use cc_risk::RiskAssessment;
use cc_types::Decision;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proposal {
    pub decision: Decision,
    pub reason: String,
}
pub struct Planner;
impl Planner {
    pub fn propose(a: &RiskAssessment, n: usize, min: usize, reached: bool) -> Proposal {
        if a.harm_rate > 0.3 {
            return Proposal {
                decision: Decision::Rollback,
                reason: "harm boundary exceeded".into(),
            };
        }
        if n < min {
            return Proposal {
                decision: Decision::Hold,
                reason: "minimum samples not reached".into(),
            };
        }
        if reached {
            return Proposal {
                decision: Decision::Promote,
                reason: "anytime-valid threshold reached".into(),
            };
        }
        Proposal {
            decision: Decision::Hold,
            reason: "evidence threshold not reached".into(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rollback() {
        let a = RiskAssessment {
            harm_rate: 0.5,
            e_value: 0.,
            incident_penalty: 0.,
            budget_remaining: 0.,
            reasons: vec![],
        };
        assert_eq!(
            Planner::propose(&a, 20, 20, false).decision,
            Decision::Rollback
        )
    }
}
