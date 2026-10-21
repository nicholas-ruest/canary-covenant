use cc_types::{CohortId, ReleaseId, SubjectId};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExposureRecord {
    pub release: ReleaseId,
    pub cohort: CohortId,
    pub subject: SubjectId,
}
#[derive(Default)]
pub struct ExposureLedger {
    seen: HashSet<(ReleaseId, CohortId, SubjectId)>,
    records: Vec<ExposureRecord>,
}
impl ExposureLedger {
    pub fn record(&mut self, r: ExposureRecord) -> bool {
        if self
            .seen
            .insert((r.release.clone(), r.cohort.clone(), r.subject.clone()))
        {
            self.records.push(r);
            true
        } else {
            false
        }
    }
    pub fn total(&self) -> usize {
        self.records.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unique() {
        let r = ExposureRecord {
            release: ReleaseId::new("r").unwrap(),
            cohort: CohortId::new("c").unwrap(),
            subject: SubjectId::new("s").unwrap(),
        };
        let mut l = ExposureLedger::default();
        assert!(l.record(r.clone()));
        assert!(!l.record(r));
        assert_eq!(l.total(), 1)
    }
}
