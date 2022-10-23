use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum PublicParameter {
    R,
    S,
}

#[derive(Debug, Clone)]
pub struct InvalidPublicParameter(pub PublicParameter);

impl Display for InvalidPublicParameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let label = match self.0 {
            PublicParameter::R => "r",
            PublicParameter::S => "s",
        };

        write!(f, "The {} must be greater than zero and less than q parameter", label)
    }
}

impl Error for InvalidPublicParameter {}