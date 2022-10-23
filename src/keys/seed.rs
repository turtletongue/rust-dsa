use num::bigint::{BigUint, RandBigInt};
use num::Zero;

use crate::constants::N;

pub fn generate_seed(length: u32) -> BigUint {
    assert!(length >= N, "Seed length must be greater than or equal to {}", N);

    let seed_bound: BigUint = BigUint::from(2_u8).pow(N - 1);
    let mut seed = BigUint::zero();

    let mut rng = rand::thread_rng();

    while seed < seed_bound {
        seed = BigUint::from(rng.gen_biguint(length as u64));
    }

    seed
}