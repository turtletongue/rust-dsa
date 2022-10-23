use std::error::Error;
use num::bigint::RandomBits;
use num::{BigUint, Integer, One};
use rand::Rng;

use crate::constants::N;
use crate::utils::get_inverted_number;

pub fn generate_secret_number(q_prime: &BigUint) -> Result<(BigUint, BigUint), Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    let c: BigUint = rng.sample(RandomBits::new((N + 64) as u64));
    let k = c.mod_floor(&(q_prime.clone() - BigUint::one())) + BigUint::one();

    let inverted_k = get_inverted_number(k.clone(), q_prime.clone())?;

    Ok((k, inverted_k))
}