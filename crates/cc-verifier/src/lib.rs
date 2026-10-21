use cc_ledger::AuditLedger;
use cc_rvf::{WitnessArtifact, WitnessChain};
use cc_types::Decision;
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerificationReport {
    pub valid: bool,
    pub checks: Vec<String>,
}
#[derive(Debug, Error)]
pub enum VerifyError {
    #[error("ledger invalid")]
    Ledger,
    #[error("witness invalid")]
    Witness,
    #[error("integrations missing")]
    Integrations,
}
pub struct Verifier;
impl Verifier {
    pub fn verify(
        d: Decision,
        l: &AuditLedger,
        w: Option<&WitnessArtifact>,
        i: &[String],
    ) -> Result<VerificationReport, VerifyError> {
        if !l.valid() {
            return Err(VerifyError::Ledger);
        }
        WitnessChain::verify(w.ok_or(VerifyError::Witness)?).map_err(|_| VerifyError::Witness)?;
        if d == Decision::Promote
            && !["openfeature", "ruvector", "rvf", "eprocess"]
                .iter()
                .all(|x| i.iter().any(|y| y == x))
        {
            return Err(VerifyError::Integrations);
        }
        Ok(VerificationReport {
            valid: true,
            checks: vec!["ledger".into(), "rvf".into(), "integrations".into()],
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cc_types::ReleaseId;
    #[test]
    fn requires_witness() {
        let mut l = AuditLedger::default();
        l.append(ReleaseId::new("r").unwrap(), "a", "b");
        assert!(Verifier::verify(Decision::Hold, &l, None, &[]).is_err())
    }
}
