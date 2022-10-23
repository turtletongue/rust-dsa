use std::error::Error;
use num::{BigUint, ToPrimitive, One};
use num::bigint::RandBigInt;

use crate::constants::{L, N};
use crate::params::Params;
use super::prime_numbers::{get_checked_prime_candidate, generate_prime_number, PrimeNumber};
use super::seed::generate_seed;


pub fn generate_params(seed_length: u32) -> Result<Params, Box<dyn Error>> {
    let seed = generate_seed(seed_length);
    let q = generate_prime_number(N, seed)?;
    let p_base = generate_prime_number((L as f32 / 2_f32).ceil().to_u32().unwrap() + 1, q.prime_seed)?;

    let iterations = (L as f32 / 2_f32).ceil().to_u32().unwrap() - 1;

    let p = get_checked_prime_candidate(L, iterations,  PrimeNumber {
        prime: q.prime.clone() * p_base.prime,
        prime_seed: p_base.prime_seed,
        prime_gen_counter: p_base.prime_gen_counter,
    })?;

    let g = get_generator(&p.prime, &q.prime);

    Ok(Params { p: p.prime, q: q.prime, g })
}

fn get_generator(p: &BigUint, q: &BigUint) -> BigUint {
    let e = (p.clone() - BigUint::one()) / q;

    let mut rng = rand::thread_rng();

    loop {
        let h = rng.gen_biguint_range(&BigUint::one(), &(p - BigUint::one()));
        let g = h.modpow(&e, &p);

        if g != BigUint::one() {
            return g;
        }
    }
}