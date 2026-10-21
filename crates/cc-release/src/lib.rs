use cc_types::{Decision, ReleaseId};
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReleaseState {
    Draft,
    Validated,
    Active,
    Promoted,
    RolledBack,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseCandidate {
    pub id: ReleaseId,
    pub version: String,
    pub digest: String,
    pub state: ReleaseState,
}
#[derive(Debug, Error)]
pub enum ReleaseError {
    #[error("empty version")]
    Empty,
    #[error("invalid digest")]
    Digest,
    #[error("invalid transition")]
    Transition,
}
impl ReleaseCandidate {
    pub fn new(
        id: ReleaseId,
        version: impl Into<String>,
        digest: impl Into<String>,
    ) -> Result<Self, ReleaseError> {
        let version = version.into();
        let digest = digest.into();
        if version.trim().is_empty() {
            return Err(ReleaseError::Empty);
        }
        if digest.len() != 64
            || !digest
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(ReleaseError::Digest);
        }
        Ok(Self {
            id,
            version,
            digest,
            state: ReleaseState::Draft,
        })
    }
    pub fn validate(&mut self) -> Result<(), ReleaseError> {
        if self.state != ReleaseState::Draft {
            return Err(ReleaseError::Transition);
        }
        self.state = ReleaseState::Validated;
        Ok(())
    }
    pub fn activate(&mut self) -> Result<(), ReleaseError> {
        if self.state != ReleaseState::Validated {
            return Err(ReleaseError::Transition);
        }
        self.state = ReleaseState::Active;
        Ok(())
    }
    pub fn finish(&mut self, d: Decision) -> Result<(), ReleaseError> {
        if self.state != ReleaseState::Active {
            return Err(ReleaseError::Transition);
        }
        self.state = match d {
            Decision::Promote => ReleaseState::Promoted,
            Decision::Rollback => ReleaseState::RolledBack,
            Decision::Hold => ReleaseState::Active,
        };
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lifecycle() {
        let mut r =
            ReleaseCandidate::new(ReleaseId::new("r").unwrap(), "1", "a".repeat(64)).unwrap();
        assert!(r.activate().is_err());
        r.validate().unwrap();
        r.activate().unwrap();
        r.finish(Decision::Promote).unwrap();
        assert_eq!(r.state, ReleaseState::Promoted)
    }
}
