use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::keys::generate_keys;

pub fn run_keys_generation(output_directory: &PathBuf) {
    let keys = match generate_keys() {
        Ok(keys) => keys,
        Err(error) => return println!("{error}"),
    };

    let mut private_key_file = match File::create(output_directory.join("private_key")) {
        Ok(file) => file,
        Err(_) => return println!("Failed to create file for private key, check if directory exists"),
    };

    private_key_file.write_all(&keys.private_key.to_string().into_bytes()).expect("Failed to write private key to file");

    let mut public_key_file = match File::create(output_directory.join("public_key")) {
        Ok(file) => file,
        Err(_) => return println!("Failed to create file for public key"),
    };

    let formatted_params = format!(" {} {} {}", keys.params.p, keys.params.q, keys.params.g);
    public_key_file.write_all(&(keys.public_key.to_string() + formatted_params.as_str()).into_bytes()).expect("Failed to write public key to file");

    println!("Success!");
}
