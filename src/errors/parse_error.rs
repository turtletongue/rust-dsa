use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum ParsingSubject {
    PublicKey,
    PrivateKey,
    File,
    P,
    Q,
    G,
}

#[derive(Debug, Clone)]
pub struct ParseError(pub ParsingSubject);

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let label = match self.0 {
            ParsingSubject::PublicKey => "public key",
            ParsingSubject::PrivateKey => "private key",
            ParsingSubject::File => "file",
            ParsingSubject::P => "p",
            ParsingSubject::Q => "q",
            ParsingSubject::G => "g",
        };

        write!(f, "Failed to parse {}", label)
    }
}

impl Error for ParseError {}