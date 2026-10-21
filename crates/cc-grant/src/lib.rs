use cc_types::{CohortId, Percentage, ReleaseId};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrantState {
    Issued,
    Consumed,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolloutGrant {
    pub id: String,
    pub release: ReleaseId,
    pub cohort: CohortId,
    pub ceiling: Percentage,
    pub decision_hash: String,
    pub state: GrantState,
}
#[derive(Debug, Error, PartialEq, Eq)]
pub enum GrantError {
    #[error("consumed")]
    Consumed,
    #[error("empty decision")]
    Decision,
}
impl RolloutGrant {
    pub fn issue(
        release: ReleaseId,
        cohort: CohortId,
        ceiling: Percentage,
        d: String,
    ) -> Result<Self, GrantError> {
        if d.is_empty() {
            return Err(GrantError::Decision);
        }
        let mut h = Sha256::new();
        h.update(release.as_str());
        h.update(cohort.as_str());
        h.update(&d);
        Ok(Self {
            id: hex::encode(h.finalize()),
            release,
            cohort,
            ceiling,
            decision_hash: d,
            state: GrantState::Issued,
        })
    }
    pub fn consume(&mut self) -> Result<(), GrantError> {
        if self.state == GrantState::Consumed {
            return Err(GrantError::Consumed);
        }
        self.state = GrantState::Consumed;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_use() {
        let mut g = RolloutGrant::issue(
            ReleaseId::new("r").unwrap(),
            CohortId::new("c").unwrap(),
            Percentage::new(0.1).unwrap(),
            "d".into(),
        )
        .unwrap();
        g.consume().unwrap();
        assert_eq!(g.consume(), Err(GrantError::Consumed))
    }
}
