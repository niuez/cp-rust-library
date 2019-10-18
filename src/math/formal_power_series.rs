use crate::math::modint::*;
use crate::math::convolution::numeric_theoretic_transform::*;

pub struct FormalPowerSeries<M: NttMod> {
    coef: Vec<ModInt<M>>
}

impl<M: NttMod> Clone for FormalPowerSeries<M> {
    fn clone(&self) -> Self { FormalPowerSeries { coef: self.coef.clone() } }
}

impl<M: NttMod> FormalPowerSeries<M> {
    pub fn new(coef: &[ModInt<M>]) -> Self {
        let mut coef = coef.to_vec();
        let n = (0usize.count_zeros()
                 - coef.len().leading_zeros()
                 - if coef.len().count_ones() == 1 { 1 } else { 0 }
                 ) as usize;
        coef.resize(1 << n, M::new(0));
        FormalPowerSeries { coef }
    }
    pub fn len(&self) -> usize { self.coef.len() }
    pub fn pre(mut self, d: usize) -> Self {
        self.coef.resize(d, M::new(0));
        self
    }

    pub fn inv(&self) -> Self {
        let mut g = FormalPowerSeries::new(&[self[0].inv()]);
        for i in 0..self.len().trailing_zeros() {
            g = (g.clone() * M::new(2) - g.clone() * g * self.clone().pre(1 << (i + 1))).pre(1 << (i + 1));
        }
        g.pre(self.len())
    }

    pub fn inv2(&self) -> Self {
        let mut g = FormalPowerSeries::new(&[self[0].inv()]);
        let n = self.len();
        for i in 0..self.len().trailing_zeros() {
            g = g.pre(1 << (i + 1));
            let mut ft = numeric_theoretic_transform(&self.clone().pre(1 << (i + 1)).coef);
            let gt = numeric_theoretic_transform(&g.coef);
            for j in 0..(1 << (i + 1)) { ft[j] *= gt[j]; }
            let mut e = inverse_numeric_theoretic_transform(&ft);
            for j in 0..(1 << i) {
                e[j] = ModInt::new(0);
                e[j + (1 << i)] = e[j + (1 << i)] * ModInt::newi(-1);
            }
            let mut et = numeric_theoretic_transform(&e);
            for j in 0..(1 << (i + 1)) { et[j] *= gt[j] }
            let mut e = inverse_numeric_theoretic_transform(&et);
            for j in 0..(1 << i) { e[j] = g[j] }
            g.coef = e;
        }
        g.pre(n)
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
        for i in 0..rhs.len() { next[i] -= rhs[i]; }
        next
    }
}

impl<M: NttMod> std::ops::Mul for FormalPowerSeries<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let n = std::cmp::max(self.len(), rhs.len()) << 1;
        let mut a = numeric_theoretic_transform(&self.pre(n).coef);
        let b = numeric_theoretic_transform(&rhs.pre(n).coef);
        for i in 0..n { a[i] *= b[i]; }
        FormalPowerSeries { coef: inverse_numeric_theoretic_transform(&a) }
    }
}

impl<M: NttMod> std::ops::Mul<ModInt<M>> for FormalPowerSeries<M> {
    type Output = Self;
    fn mul(mut self, rhs: ModInt<M>) -> Self {
        for i in 0..self.len() { self[i] *= rhs; }
        self
    }
}

#[test]
fn inv_test() {
    type M = NttMod976224257;
    type P = FormalPowerSeries<M>;
    let p = P::new(&[ModInt::new(1), ModInt::newi(-1)]).pre(16);
    assert_eq!(p.inv().coef.iter().map(|x| x.value()).collect::<Vec<_>>(), vec![1; 16]);
}

#[test]
fn inv2_test() {
    type M = NttMod976224257;
    type P = FormalPowerSeries<M>;
    let p = P::new(&[ModInt::new(1), ModInt::newi(-1)]).pre(16);
    assert_eq!(p.inv2().coef.iter().map(|x| x.value()).collect::<Vec<_>>(), vec![1; 16]);
}
