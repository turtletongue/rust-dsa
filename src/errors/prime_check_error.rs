use std::error::Error;
use std::fmt::{Display, Formatter};
use num::BigUint;

#[derive(Debug, Clone)]
pub struct PrimeCheckError(pub BigUint);

impl Display for PrimeCheckError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "prime check failed for number: {}", self.0)
    }
}

impl Error for PrimeCheckError {}