use cc_types::{CohortId, Percentage};
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CohortStage {
    pub id: CohortId,
    pub ceiling: Percentage,
}
#[derive(Clone, Debug)]
pub struct CohortPlan {
    stages: Vec<CohortStage>,
    cursor: usize,
}
#[derive(Debug, Error)]
pub enum PlanError {
    #[error("invalid plan")]
    Invalid,
    #[error("complete")]
    Complete,
}
impl CohortPlan {
    pub fn new(stages: Vec<CohortStage>) -> Result<Self, PlanError> {
        if stages.is_empty()
            || stages
                .windows(2)
                .any(|x| x[0].ceiling.get() > x[1].ceiling.get())
        {
            Err(PlanError::Invalid)
        } else {
            Ok(Self { stages, cursor: 0 })
        }
    }
    pub fn current(&self) -> &CohortStage {
        &self.stages[self.cursor]
    }
    pub fn advance(&mut self) -> Result<&CohortStage, PlanError> {
        if self.cursor + 1 == self.stages.len() {
            return Err(PlanError::Complete);
        }
        self.cursor += 1;
        Ok(self.current())
    }
}
pub fn standard() -> CohortPlan {
    CohortPlan::new(vec![
        CohortStage {
            id: CohortId::new("internal").unwrap(),
            ceiling: Percentage::new(0.05).unwrap(),
        },
        CohortStage {
            id: CohortId::new("beta").unwrap(),
            ceiling: Percentage::new(0.25).unwrap(),
        },
        CohortStage {
            id: CohortId::new("production").unwrap(),
            ceiling: Percentage::new(1.).unwrap(),
        },
    ])
    .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordered() {
        let mut p = standard();
        assert_eq!(p.current().id.as_str(), "internal");
        assert_eq!(p.advance().unwrap().id.as_str(), "beta")
    }
}
