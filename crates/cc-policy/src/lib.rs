use cc_planner::Proposal;
use cc_risk::RiskAssessment;
use cc_types::Decision;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyInput {
    pub flag: bool,
    pub rvf: bool,
    pub ruvector: bool,
    pub eprocess: bool,
    pub proposal: Proposal,
    pub risk: RiskAssessment,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub decision: Decision,
    pub reasons: Vec<String>,
}
pub struct PolicyEngine;
impl PolicyEngine {
    pub fn decide(i: PolicyInput) -> PolicyDecision {
        if i.proposal.decision == Decision::Rollback {
            return PolicyDecision {
                decision: Decision::Rollback,
                reasons: vec![i.proposal.reason],
            };
        }
        let mut r = vec![];
        if !i.flag {
            r.push("OpenFeature denied".into())
        }
        if !i.rvf {
            r.push("RVF unavailable".into())
        }
        if !i.ruvector {
            r.push("RuVector unavailable".into())
        }
        if !i.eprocess {
            r.push("e-process unavailable".into())
        }
        if i.risk.incident_penalty >= 1. {
            r.push("critical incident".into())
        }
        if i.proposal.decision != Decision::Promote {
            r.push(i.proposal.reason)
        }
        if r.is_empty() {
            PolicyDecision {
                decision: Decision::Promote,
                reasons: vec!["all gates passed".into()],
            }
        } else {
            PolicyDecision {
                decision: Decision::Hold,
                reasons: r,
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flag_dominates() {
        let r = RiskAssessment {
            harm_rate: 0.,
            e_value: 20.,
            incident_penalty: 0.,
            budget_remaining: 0.2,
            reasons: vec![],
        };
        assert_eq!(
            PolicyEngine::decide(PolicyInput {
                flag: false,
                rvf: true,
                ruvector: true,
                eprocess: true,
                proposal: Proposal {
                    decision: Decision::Promote,
                    reason: "ok".into()
                },
                risk: r
            })
            .decision,
            Decision::Hold
        )
    }
}
