use std::fs;
use std::error::Error;
use std::path::PathBuf;
use num::bigint::ToBigInt;
use num::{BigInt, BigUint, Integer, Zero, One};

use crate::errors::{ParseError, InvalidPublicKeyFileError, NumberInversionError, ParsingSubject};
use crate::params::Params;

pub fn get_inverted_number(number: BigUint, parameter: BigUint) -> Result<BigUint, NumberInversionError> {
    assert!(number > BigUint::one(), "The number for inversion must be a positive integer");
    assert!(number < parameter, "The number for inversion must be less than parameter");

    let mut i = parameter.clone().to_bigint().unwrap();
    let mut j = number.to_bigint().unwrap();
    let mut y1 = BigInt::one();
    let mut y2 = BigInt::zero();

    while j > BigInt::zero() {
        let quotient = i.div_floor(&j);
        let remainder = i - (j.clone() * quotient.clone());

        let y = y2 - (y1.clone() * quotient);

        i = j;
        j = remainder;
        y2 = y1;
        y1 = y;
    }

    if i != BigInt::one() {
        return Err(NumberInversionError(number));
    }

    Ok(y2.mod_floor(&parameter.to_bigint().unwrap()).to_biguint().unwrap())
}

pub fn parse_public_key_file(path: PathBuf) -> Result<(BigUint, Params), Box<dyn Error>> {
    let data: Vec<String> = parse_file(path)?.split(" ").map(|param| param.to_string()).collect();

    if data.len() < 3 {
        return Err(InvalidPublicKeyFileError.into());
    }

    let public_key = BigUint::parse_bytes(&data[0].clone().into_bytes(), 10).ok_or(ParseError(ParsingSubject::PublicKey))?;

    let params = Params {
        p: BigUint::parse_bytes(&data[1].clone().into_bytes(), 10).ok_or(ParseError(ParsingSubject::P))?,
        q: BigUint::parse_bytes(&data[2].clone().into_bytes(), 10).ok_or(ParseError(ParsingSubject::Q))?,
        g: BigUint::parse_bytes(&data[3].clone().into_bytes(), 10).ok_or(ParseError(ParsingSubject::G))?,
    };

    Ok((public_key, params))
}

pub fn parse_private_key_file(path: PathBuf) -> Result<BigUint, ParseError> {
    Ok(BigUint::parse_bytes(&parse_file(path)?.into_bytes(), 10).ok_or(ParseError(ParsingSubject::PrivateKey))?)
}

fn parse_file(path: PathBuf) -> Result<String, ParseError> {
    fs::read_to_string(path).map_err(|_error| ParseError(ParsingSubject::File))
}