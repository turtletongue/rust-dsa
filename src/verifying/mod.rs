use std::error::Error;
use num::{BigUint, Integer, Zero};
use sha256;

use crate::params::Params;
use crate::utils::get_inverted_number;
use crate::errors::{InvalidPublicParameter, InvalidSignatureError, PublicParameter};

pub fn verify(plaintext: String, public_key: BigUint, params: Params, r: BigUint, s: BigUint) -> Result<(), Box<dyn Error>> {
    if r < BigUint::zero() || r > params.q {
        return Err(InvalidPublicParameter(PublicParameter::R).into());
    }

    if s < BigUint::zero() || s > params.q {
        return Err(InvalidPublicParameter(PublicParameter::S).into());
    }

    let w = get_inverted_number(s, params.q.clone())?;
    let z = BigUint::parse_bytes(&sha256::digest(plaintext).into_bytes(), 16).unwrap();

    let u1 = (z * w.clone()).mod_floor(&params.q);
    let u2 = (r.clone() * w).mod_floor(&params.q);

    let v = (params.g.modpow(&u1, &params.p) * public_key.modpow(&u2, &params.p)).mod_floor(&params.p).mod_floor(&params.q);

    if v != r {
        return Err(InvalidSignatureError.into());
    }

    Ok(())
}