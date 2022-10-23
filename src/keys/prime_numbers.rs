use std::error::Error;
use std::ops::BitXor;
use num::bigint::{BigUint};
use num::integer::Roots;
use num::{Integer, ToPrimitive, Zero, One};
use sha256;

use crate::constants::N;
use crate::errors::PrimeCheckError;
use super::utils::{generate_pseudo_random, get_bounded};

pub struct PrimeNumber {
    pub prime: BigUint,
    pub prime_seed: BigUint,
    pub prime_gen_counter: i32,
}

pub fn generate_prime_number(length: u32, seed: BigUint) -> Result<PrimeNumber, Box<dyn Error>> {
    assert!(length > 1, "Prime number must have length greater than one");

    if length >= 33 {
        let out_prime_number = generate_prime_number((length as f32 / 2_f32).ceil().to_u32().unwrap() + 1, seed.clone())?;

        let iterations = (length as f32 / N as f32).ceil().to_u32().unwrap() - 1;

        return Ok(get_checked_prime_candidate(length, iterations, out_prime_number)?);
    }

    let mut prime_seed = seed;
    let mut prime_gen_counter = 0;

    loop {
        let first_digest = sha256::digest(prime_seed.clone().to_string()).into_bytes();
        let second_digest = sha256::digest((prime_seed.clone() + BigUint::one()).to_string()).into_bytes();

        let c = BigUint::parse_bytes(&first_digest, 16).unwrap().bitxor(BigUint::parse_bytes(&second_digest, 16).unwrap());
        let c = 2 * get_bounded(c, length).to_u64().unwrap() / 2 + 1;

        prime_gen_counter += 1;
        prime_seed += BigUint::from(2_u8);

        if is_prime(c) {
            return Ok(PrimeNumber {
                prime: BigUint::from(c),
                prime_seed,
                prime_gen_counter,
            });
        }
    }
}

pub fn get_checked_prime_candidate(length: u32, iterations: u32, prime_number: PrimeNumber) -> Result<PrimeNumber, PrimeCheckError> {
    let PrimeNumber { prime, mut prime_seed, mut prime_gen_counter } = prime_number;
    let old_counter = prime_gen_counter;

    let x = get_bounded(generate_pseudo_random(&prime_seed, iterations), length);
    prime_seed += iterations + 1;

    let mut t = x.div_ceil(&(BigUint::from(2_u8) * prime.clone()));

    loop {
        if BigUint::from(2_u8) * t.clone() * prime.clone() + BigUint::one() > BigUint::from(2_u8).pow(length) {
            t = BigUint::from(2_u8).pow(length - 1).div_ceil(&(BigUint::from(2_u8) * prime.clone()));
        }

        let c = BigUint::from(2_u8) * t.clone() * prime.clone() + BigUint::one();
        prime_gen_counter += 1;

        let mut a = generate_pseudo_random(&prime_seed, iterations);
        prime_seed += iterations + 1;
        a = BigUint::from(2_u8) + a.mod_floor(&(c.clone() - BigUint::from(3_u8)));

        let z = a.modpow(&(BigUint::from(2_u8) * t.clone()), &c);

        let is_ok = get_greatest_common_divisor(z.clone() - BigUint::one(), c.clone()) == BigUint::one() && z.modpow(&prime, &c) == BigUint::one();

        if is_ok {
            return Ok(PrimeNumber {
                prime: c,
                prime_seed,
                prime_gen_counter,
            });
        }

        if prime_gen_counter >= ((4 * length as i32) + old_counter) {
            return Err(PrimeCheckError(c));
        }

        t += BigUint::one();
    }
}

fn is_prime(number: u64) -> bool {
    for i in 2..number.sqrt() {
        if number % i == 0 {
            return false;
        }
    }

    true
}

fn get_greatest_common_divisor(a: BigUint, b: BigUint) -> BigUint {
    let mut a = a;
    let mut b = b;

    while a.mod_floor(&b) > BigUint::zero() {
        let remainder = a.mod_floor(&b);

        a = b;
        b = remainder;
    }

    b.clone()
}