use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Observation {
    pub sequence: u64,
    pub harm: bool,
    pub latency_ms: f64,
    pub success: bool,
}
#[derive(Debug, Error)]
pub enum MetricError {
    #[error("invalid latency")]
    Latency,
    #[error("sequence not increasing")]
    Sequence,
}
impl Observation {
    pub fn new(
        sequence: u64,
        harm: bool,
        latency_ms: f64,
        success: bool,
    ) -> Result<Self, MetricError> {
        if !latency_ms.is_finite() || latency_ms < 0. {
            Err(MetricError::Latency)
        } else {
            Ok(Self {
                sequence,
                harm,
                latency_ms,
                success,
            })
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObservationBatch(pub Vec<Observation>);
impl ObservationBatch {
    pub fn new(v: Vec<Observation>) -> Result<Self, MetricError> {
        if v.windows(2).any(|x| x[0].sequence >= x[1].sequence) {
            Err(MetricError::Sequence)
        } else {
            Ok(Self(v))
        }
    }
    pub fn harm_rate(&self) -> f64 {
        if self.0.is_empty() {
            0.
        } else {
            self.0.iter().filter(|x| x.harm).count() as f64 / self.0.len() as f64
        }
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn summary() {
        let b = ObservationBatch::new(vec![Observation::new(1, true, 2., false).unwrap()]).unwrap();
        assert_eq!(b.harm_rate(), 1.)
    }
}
