use cc_incidents::Incident;
use cc_metrics::ObservationBatch;
pub struct IncidentEmbedder;
impl IncidentEmbedder {
    pub fn incident(i: &Incident) -> Vec<f32> {
        i.features.to_vec()
    }
    pub fn batch(b: &ObservationBatch) -> Vec<f32> {
        if b.is_empty() {
            return vec![0.; 4];
        }
        let n = b.len() as f32;
        let h = b.0.iter().filter(|x| x.harm).count() as f32 / n;
        let l = (b.0.iter().map(|x| x.latency_ms).sum::<f64>() / b.len() as f64 / 1000.)
            .clamp(0., 1.) as f32;
        let f = b.0.iter().filter(|x| !x.success).count() as f32 / n;
        vec![h, l, f, h]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cc_metrics::Observation;
    #[test]
    fn dimension() {
        let b =
            ObservationBatch::new(vec![Observation::new(1, false, 100., true).unwrap()]).unwrap();
        assert_eq!(IncidentEmbedder::batch(&b), vec![0., 0.1, 0., 0.])
    }
}
