use crate::math::modint::*;
use crate::math::convolution::numeric_theoretic_transform::*;

pub struct FormalPowerSeries<M: NttMod> {
    coef: Vec<ModInt<M>>
}

impl<M: NttMod> FormalPowerSeries<M> {
    pub fn new(coef: &[ModInt<M>]) -> Self {
        let mut coef = coef.to_vec();
        let n = (0usize.count_zeros()
                 - coef.len().leading_zeros()
                 - if coef.len().count_ones() == 1 { 1 } else { 0 }
                 ) as usize;
        coef.resize(n, M::new(0));
        FormalPowerSeries { coef }
    }
    pub fn len(&self) -> usize { self.coef.len() }
    pub fn pre(mut self, d: usize) -> Self {
        self.coef.resize(d, M::new(0));
        self
    }
}

impl<M: NttMod> std::ops::Index<usize> for FormalPowerSeries<M> {
    type Output = ModInt<M>;
    fn index(&self, i: usize) -> &Self::Output { &self.coef[i] }
}

impl<M: NttMod> std::ops::IndexMut<usize> for FormalPowerSeries<M> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output { &mut self.coef[i] }
}

impl<M: NttMod> std::ops::Add for FormalPowerSeries<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let n = std::cmp::max(self.len(), rhs.len());
        let mut next = self.pre(n);
        for i in 0..rhs.len() { next[i] += rhs[i]; }
        next
    }
}

impl<M: NttMod> std::ops::Sub for FormalPowerSeries<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let n = std::cmp::max(self.len(), rhs.len());
        let mut next = self.pre(n);
        for i in 0..rhs.len() { next[i] += rhs[i]; }
        next
    }
}

impl<M: NttMod> std::ops::Mul for FormalPowerSeries<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let n = std::cmp::max(self.len(), rhs.len());
        unimplemented!();
    }
}
