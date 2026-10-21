use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ValueError {
    #[error("value must not be empty")]
    Empty,
    #[error("percentage must be finite and in [0,1]")]
    Percentage,
}
macro_rules! id {
    ($n:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $n(String);
        impl $n {
            pub fn new(v: impl Into<String>) -> Result<Self, ValueError> {
                let v = v.into();
                if v.trim().is_empty() {
                    Err(ValueError::Empty)
                } else {
                    Ok(Self(v))
                }
            }
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}
id!(ReleaseId);
id!(CohortId);
id!(SubjectId);
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Percentage(f64);
impl Percentage {
    pub fn new(v: f64) -> Result<Self, ValueError> {
        if v.is_finite() && (0.0..=1.0).contains(&v) {
            Ok(Self(v))
        } else {
            Err(ValueError::Percentage)
        }
    }
    pub fn get(self) -> f64 {
        self.0
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Decision {
    Hold,
    Promote,
    Rollback,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invariants() {
        assert!(ReleaseId::new(" ").is_err());
        assert!(Percentage::new(f64::NAN).is_err());
        assert_eq!(Percentage::new(1.0).unwrap().get(), 1.0)
    }
}
