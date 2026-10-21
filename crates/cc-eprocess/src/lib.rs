use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EProcess {
    p0: f64,
    alpha: f64,
    lambda: f64,
    wealth: f64,
    factors: Vec<f64>,
}
#[derive(Debug, Error)]
pub enum EvidenceError {
    #[error("invalid parameters")]
    Parameters,
    #[error("invalid wealth")]
    Wealth,
}
impl EProcess {
    pub fn new(p0: f64, alpha: f64, lambda: f64) -> Result<Self, EvidenceError> {
        if !(0.0..1.0).contains(&p0)
            || !(0.0..1.0).contains(&alpha)
            || lambda > 0.
            || lambda < -1. / (1. - p0)
        {
            return Err(EvidenceError::Parameters);
        }
        Ok(Self {
            p0,
            alpha,
            lambda,
            wealth: 1.,
            factors: vec![],
        })
    }
    pub fn observe(&mut self, harm: bool) -> Result<f64, EvidenceError> {
        let x = if harm { 1. } else { 0. };
        let f = 1. + self.lambda * (x - self.p0);
        self.wealth *= f;
        if !self.wealth.is_finite() || self.wealth < 0. {
            return Err(EvidenceError::Wealth);
        }
        self.factors.push(f);
        Ok(self.wealth)
    }
    pub fn wealth(&self) -> f64 {
        self.wealth
    }
    pub fn reached(&self) -> bool {
        self.wealth >= 1. / self.alpha
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn healthy_crosses() {
        let mut e = EProcess::new(0.2, 0.1, -0.5).unwrap();
        for _ in 0..30 {
            e.observe(false).unwrap();
        }
        assert!(e.reached())
    }
    #[test]
    fn harm_reduces() {
        let mut e = EProcess::new(0.2, 0.1, -0.5).unwrap();
        e.observe(true).unwrap();
        assert!(e.wealth() < 1.)
    }
}
