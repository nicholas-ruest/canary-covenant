use cc_types::ReleaseId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecoveryAction {
    pub key: String,
    pub operation: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecoveryPlan {
    pub release: ReleaseId,
    pub last_good_digest: String,
    pub actions: Vec<RecoveryAction>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryOutcome {
    Applied,
    AlreadyApplied,
}
#[derive(Debug, Error)]
pub enum RecoveryError {
    #[error("bad digest")]
    Digest,
}
impl RecoveryPlan {
    pub fn new(release: ReleaseId, d: String) -> Result<Self, RecoveryError> {
        if d.len() != 64 || !d.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(RecoveryError::Digest);
        }
        Ok(Self {
            release,
            last_good_digest: d,
            actions: vec![
                RecoveryAction {
                    key: "freeze".into(),
                    operation: "freeze exposure".into(),
                },
                RecoveryAction {
                    key: "restore".into(),
                    operation: "restore last good".into(),
                },
            ],
        })
    }
}
#[derive(Default)]
pub struct RecoveryExecutor {
    applied: HashSet<String>,
}
impl RecoveryExecutor {
    pub fn execute(&mut self, p: &RecoveryPlan) -> Vec<RecoveryOutcome> {
        p.actions
            .iter()
            .map(|a| {
                if self
                    .applied
                    .insert(format!("{}:{}", p.release.as_str(), a.key))
                {
                    RecoveryOutcome::Applied
                } else {
                    RecoveryOutcome::AlreadyApplied
                }
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn idempotent() {
        let p = RecoveryPlan::new(ReleaseId::new("r").unwrap(), "a".repeat(64)).unwrap();
        let mut e = RecoveryExecutor::default();
        assert_eq!(e.execute(&p)[0], RecoveryOutcome::Applied);
        assert_eq!(e.execute(&p)[0], RecoveryOutcome::AlreadyApplied)
    }
}
