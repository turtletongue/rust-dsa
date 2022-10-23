use std::path::PathBuf;
use num::BigUint;

use crate::utils::parse_public_key_file;
use crate::verifying::verify;

pub fn run_verifying(public_key_path: &PathBuf, message: String, r: String, s: String) {
    let (public_key, params) = match parse_public_key_file(public_key_path.into()) {
        Ok(result) => result,
        Err(error) => return println!("{error}"),
    };

    let r = match BigUint::parse_bytes(&r.into_bytes(), 10) {
        Some(r) => r,
        None => return println!("Failed to parse r"),
    };

    let s = match BigUint::parse_bytes(&s.into_bytes(), 10) {
        Some(s) => s,
        None => return println!("Failed to parse s"),
    };

    match verify(message, public_key, params, r, s) {
        Ok(_) => println!("Successfully verified!"),
        Err(err) => println!("{err}"),
    }
}
