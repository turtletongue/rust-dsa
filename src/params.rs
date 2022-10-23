use num::BigUint;

pub struct Params {
    pub p: BigUint,
    pub q: BigUint,
    pub g: BigUint,
}

impl Clone for Params {
    fn clone(&self) -> Self {
        Params { p: self.p.clone(), q: self.q.clone(), g: self.g.clone() }
    }
}
