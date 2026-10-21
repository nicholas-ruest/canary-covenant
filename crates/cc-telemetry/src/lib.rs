use cc_types::ReleaseId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub name: String,
    pub release: ReleaseId,
    pub sequence: u64,
    pub fields: HashMap<String, String>,
}
#[derive(Default)]
pub struct Telemetry {
    events: Vec<TelemetryEvent>,
}
impl Telemetry {
    pub fn emit(&mut self, name: &str, release: ReleaseId, fields: HashMap<String, String>) {
        self.events.push(TelemetryEvent {
            name: name.into(),
            release,
            sequence: self.events.len() as u64 + 1,
            fields,
        })
    }
    pub fn count(&self, name: &str) -> usize {
        self.events.iter().filter(|x| x.name == name).count()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn counts() {
        let mut t = Telemetry::default();
        t.emit("decision", ReleaseId::new("r").unwrap(), HashMap::new());
        assert_eq!(t.count("decision"), 1)
    }
}
