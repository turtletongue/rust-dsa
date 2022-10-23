use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct InvalidPublicKeyFileError;

impl Display for InvalidPublicKeyFileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid public key file")
    }
}

impl Error for InvalidPublicKeyFileError {}