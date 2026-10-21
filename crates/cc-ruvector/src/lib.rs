use cc_incidents::{Incident, Severity};
use ruvector_core::{SearchQuery, VectorDB, VectorEntry};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IncidentMatch {
    pub id: String,
    pub severity: Severity,
    pub score: f32,
}
#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("upstream: {0}")]
    Upstream(String),
    #[error("unknown result")]
    Unknown,
    #[error("invalid score")]
    Score,
}
pub struct IncidentMemory {
    db: VectorDB,
    records: HashMap<String, Incident>,
}
impl IncidentMemory {
    pub fn open(_path: &Path) -> Result<Self, MemoryError> {
        let db = VectorDB::with_dimensions(4).map_err(|e| MemoryError::Upstream(e.to_string()))?;
        Ok(Self {
            db,
            records: HashMap::new(),
        })
    }
    pub fn seed(&mut self, x: &[Incident]) -> Result<(), MemoryError> {
        for i in x {
            self.db
                .insert(VectorEntry {
                    id: Some(i.id.clone()),
                    vector: i.features.to_vec(),
                    metadata: None,
                })
                .map_err(|e| MemoryError::Upstream(e.to_string()))?;
            self.records.insert(i.id.clone(), i.clone());
        }
        Ok(())
    }
    pub fn recall(&self, v: Vec<f32>, k: usize) -> Result<Vec<IncidentMatch>, MemoryError> {
        self.db
            .search(SearchQuery {
                vector: v,
                k,
                filter: None,
                ef_search: None,
            })
            .map_err(|e| MemoryError::Upstream(e.to_string()))?
            .into_iter()
            .map(|h| {
                if !h.score.is_finite() {
                    return Err(MemoryError::Score);
                }
                let i = self.records.get(&h.id).ok_or(MemoryError::Unknown)?;
                Ok(IncidentMatch {
                    id: h.id,
                    severity: i.severity,
                    score: h.score,
                })
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn real_roundtrip() {
        let d = tempfile::tempdir().unwrap();
        let mut m = IncidentMemory::open(&d.path().join("i.db")).unwrap();
        m.seed(&cc_incidents::seeded()).unwrap();
        assert_eq!(
            m.recall(vec![0.02, 0.15, 0.01, 0.], 1).unwrap()[0].id,
            "latency-tail"
        )
    }
}
