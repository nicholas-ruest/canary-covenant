use rvf_crypto::{
    hash::shake256_256,
    witness::{create_witness_chain, verify_witness_chain, WitnessEntry},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WitnessArtifact {
    pub bytes: Vec<u8>,
    pub entries: usize,
    pub digest_hex: String,
}
#[derive(Debug, Error)]
pub enum WitnessError {
    #[error("verification failed: {0}")]
    Verify(String),
}
pub struct WitnessChain;
impl WitnessChain {
    pub fn seal(a: &[Vec<u8>]) -> WitnessArtifact {
        let e: Vec<_> = a
            .iter()
            .enumerate()
            .map(|(i, x)| WitnessEntry {
                prev_hash: [0; 32],
                action_hash: shake256_256(x),
                timestamp_ns: 1_700_000_000_000_000_000 + i as u64,
                witness_type: 2,
            })
            .collect();
        let bytes = create_witness_chain(&e);
        let digest_hex = hex::encode(shake256_256(&bytes));
        WitnessArtifact {
            bytes,
            entries: e.len(),
            digest_hex,
        }
    }
    pub fn verify(a: &WitnessArtifact) -> Result<(), WitnessError> {
        let v = verify_witness_chain(&a.bytes).map_err(|e| WitnessError::Verify(e.to_string()))?;
        if v.len() != a.entries || hex::encode(shake256_256(&a.bytes)) != a.digest_hex {
            return Err(WitnessError::Verify("metadata mismatch".into()));
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detects_tamper() {
        let mut a = WitnessChain::seal(&[b"a".to_vec(), b"b".to_vec()]);
        WitnessChain::verify(&a).unwrap();
        a.bytes[80] ^= 1;
        assert!(WitnessChain::verify(&a).is_err())
    }
}
