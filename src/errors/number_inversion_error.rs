use std::error::Error;
use std::fmt::{Display, Formatter};
use num::BigUint;

#[derive(Debug, Clone)]
pub struct NumberInversionError(pub BigUint);

impl Display for NumberInversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "number inversion failed for number: {}", self.0)
    }
}

impl Error for NumberInversionError {}