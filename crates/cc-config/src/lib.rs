use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub alpha: f64,
    pub unacceptable_harm: f64,
    pub min_samples: usize,
    pub bet: f64,
}
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid {0}")]
    Invalid(String),
}
impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            alpha: 0.1,
            unacceptable_harm: 0.2,
            min_samples: 20,
            bet: -0.5,
        }
    }
}
impl RuntimeConfig {
    pub fn from_map(m: &HashMap<String, String>) -> Result<Self, ConfigError> {
        let mut c = Self::default();
        if let Some(v) = m.get("CANARY_ALPHA") {
            c.alpha = v
                .parse()
                .map_err(|_| ConfigError::Invalid("alpha".into()))?
        }
        if !(0.0..1.0).contains(&c.alpha) {
            return Err(ConfigError::Invalid("alpha".into()));
        }
        Ok(c)
    }
    pub fn redacted(m: &HashMap<String, String>) -> HashMap<String, String> {
        m.iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    if k.contains("TOKEN") || k.contains("KEY") {
                        "[REDACTED]".into()
                    } else {
                        v.clone()
                    },
                )
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn redacts() {
        let m = HashMap::from([("API_TOKEN".into(), "secret".into())]);
        assert_eq!(RuntimeConfig::redacted(&m)["API_TOKEN"], "[REDACTED]")
    }
}
