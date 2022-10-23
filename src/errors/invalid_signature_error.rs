use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct InvalidSignatureError;

impl Display for InvalidSignatureError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid signature")
    }
}

impl Error for InvalidSignatureError {}