mod secret_numbers;

use std::error::Error;
use num::{BigUint, Integer, Zero};
use sha256;

use crate::params::Params;
use secret_numbers::generate_secret_number;

pub fn sign(plain_text: String, private_key: BigUint, params: Params) -> Result<(BigUint, BigUint), Box<dyn Error>> {
    let mut r = BigUint::zero();
    let mut s = BigUint::zero();

    while r == BigUint::zero() || s == BigUint::zero() {
        let (k, inverted_k) = generate_secret_number(&params.q)?;

        r = params.g.modpow(&k, &params.p).mod_floor(&params.q);
        let z = BigUint::parse_bytes(&sha256::digest(plain_text.clone()).into_bytes(), 16).unwrap();
        s = (inverted_k * (z + private_key.clone() * r.clone())).mod_floor(&params.q);
    }

    Ok((r, s))
}
