pub trait Random {
    fn rand_u64(&mut self) -> u64;
}

pub trait RandomGen {
    fn rand_gen<R: Random>(rng: &mut R) -> Self;
}

pub struct Xorshift128 {
    x: u64,
    y: u64,
    z: u64,
    w: u64,
}

impl Xorshift128 {
    pub fn new(seed: u64) -> Self {
        let x = 123456789;
        let y = 362436069;
        let z = 521288629;
        Self { x, y, z, w: seed }
    }
}

impl Random for Xorshift128 {
    fn rand_u64(&mut self) -> u64 {
        let t = (self.x) ^ (self.x<<11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w>>19)) ^ (t ^ (t>>8));
        self.w
    }
}

use crate::math::modint::{ Mod, ModInt };

impl<M: Mod> RandomGen for ModInt<M> {
    fn rand_gen<R: Random>(rng: &mut R) -> Self {
        ModInt::newu64(rng.rand_u64())
    }
}
