mod prime_numbers;
mod utils;
mod params;
mod seed;

use std::error::Error;
use num::bigint::{BigUint, RandomBits};
use num::{Integer, One};
use rand::Rng;

use crate::constants::N;
use crate::params::Params;
use params::generate_params;

pub struct Keys {
    pub private_key: BigUint,
    pub public_key: BigUint,
    pub params: Params,
}

pub fn generate_keys() -> Result<Keys, Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    let params = generate_params(N)?;
    let c: BigUint = rng.sample(RandomBits::new((N + 64) as u64));

    let private_key = c.mod_floor(&(params.q.clone() - BigUint::one())) + BigUint::one();
    let public_key = params.g.clone().modpow(&private_key, &params.p);

    Ok(Keys { private_key, public_key, params: params.clone() })
}
