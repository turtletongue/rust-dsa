use std::ops::{Add, Mul};
use num::{BigUint, Integer, Zero};

use crate::constants::N;

pub fn generate_pseudo_random(prime_seed: &BigUint, iterations: u32) -> BigUint {
    let mut number = BigUint::zero();

    for i in 0..iterations {
        number += BigUint::parse_bytes(&sha256::digest(prime_seed.clone().add(i).to_string()).into_bytes(), 16).unwrap().mul(BigUint::from(2_u8).pow(i * N));
    }

    number
}

pub fn get_bounded(number: BigUint, length: u32) -> BigUint {
    let degree = BigUint::from(2_u8).pow(length - 1);

    degree.clone() + (number.mod_floor(&degree))
}