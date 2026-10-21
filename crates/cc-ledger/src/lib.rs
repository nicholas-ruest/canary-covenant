use cc_types::ReleaseId;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LedgerEvent {
    pub sequence: u64,
    pub release: ReleaseId,
    pub kind: String,
    pub detail: String,
    pub previous_hash: String,
    pub hash: String,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditLedger {
    events: Vec<LedgerEvent>,
}
impl AuditLedger {
    pub fn append(&mut self, release: ReleaseId, kind: &str, detail: &str) -> &LedgerEvent {
        let sequence = self.events.len() as u64 + 1;
        let previous_hash = self
            .events
            .last()
            .map(|x| x.hash.clone())
            .unwrap_or_else(|| "0".repeat(64));
        let mut h = Sha256::new();
        h.update(sequence.to_le_bytes());
        h.update(release.as_str());
        h.update(kind);
        h.update(detail);
        h.update(&previous_hash);
        let hash = hex::encode(h.finalize());
        self.events.push(LedgerEvent {
            sequence,
            release,
            kind: kind.into(),
            detail: detail.into(),
            previous_hash,
            hash,
        });
        self.events.last().unwrap()
    }
    pub fn events(&self) -> &[LedgerEvent] {
        &self.events
    }
    pub fn actions(&self) -> Vec<Vec<u8>> {
        self.events
            .iter()
            .map(|x| serde_json::to_vec(x).unwrap())
            .collect()
    }
    pub fn valid(&self) -> bool {
        self.events.iter().enumerate().all(|(i, e)| {
            e.sequence == i as u64 + 1
                && (if i == 0 {
                    e.previous_hash == "0".repeat(64)
                } else {
                    e.previous_hash == self.events[i - 1].hash
                })
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn links() {
        let r = ReleaseId::new("r").unwrap();
        let mut l = AuditLedger::default();
        l.append(r.clone(), "a", "x");
        l.append(r, "b", "y");
        assert!(l.valid());
        assert_eq!(l.events()[1].previous_hash, l.events()[0].hash)
    }
}
