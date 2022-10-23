use std::path::PathBuf;

use crate::signing::sign;
use crate::utils::{parse_private_key_file, parse_public_key_file};

pub fn run_signing(public_key_path: &PathBuf, private_key_path: &PathBuf, message: String) {
    let (_, params) = match parse_public_key_file(public_key_path.into()) {
        Ok(result) => result,
        Err(error) => return println!("{error}"),
    };

    let private_key = match parse_private_key_file(private_key_path.into()) {
        Ok(result) => result,
        Err(error) => return println!("{error}"),
    };

    let (r, s) = match sign(message.clone(), private_key, params) {
        Ok(result) => result,
        Err(error) => return println!("{error}"),
    };

    println!("Message = \"{}\";\nr = {};\ns = {};", message, r, s);
}