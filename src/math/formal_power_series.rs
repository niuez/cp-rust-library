use crate::math::fps_multiply::FpsMultiply;

use std::ops::{ Add, Sub, Mul, Div };


pub struct FormalPowerSeries<FM: FpsMultiply> {
    coef: Vec<FM::Target>,
    _p: std::marker::PhantomData<FM>,
}

impl<FM: FpsMultiply> Clone for FormalPowerSeries<FM> {
    fn clone(&self) -> Self { FormalPowerSeries::new_raw(self.coef.clone()) }
}

impl<FM: FpsMultiply> FormalPowerSeries<FM> {
    fn new_raw(coef: Vec<FM::Target>) -> Self { FormalPowerSeries { coef, _p: std::marker::PhantomData } }
    pub fn new(coef: &[FM::Target]) -> Self {
        FormalPowerSeries::new_raw(coef.to_vec())
    }
    pub fn len(&self) -> usize { self.coef.len() }
    pub fn bound_len(&mut self) {
        self.coef.resize(self.coef.len().next_power_of_two(), FM::Target::from(0));
    }
    pub fn pre(mut self, d: usize) -> Self {
        self.coef.resize(d, FM::Target::from(0));
        self
    }

    pub fn inv(&self) -> Self {
        let mut g = FormalPowerSeries::<FM>::new(&[FM::Target::from(1) / self[0]]);
        for i in 0..self.len().trailing_zeros() {
            g = (g.clone() * FM::Target::from(2) - g.clone() * g * self.clone().pre(1 << (i + 1))).pre(1 << (i + 1));
        }
        g.pre(self.len())
    }

    pub fn inv2(&self) -> Self {
        let mut g = FormalPowerSeries::<FM>::new(&[FM::Target::from(1) / self[0]]);
        let mut f = self.clone();
        f.bound_len();
        let n = f.len();
        for i in 0..f.len().trailing_zeros() {
            g = g.pre(1 << (i + 1));
            let gdft = FM::dft(g.coef.clone());
            let mut e = FM::idft(FM::multiply(FM::dft(f.clone().pre(1 << (i + 1)).coef), gdft.clone()));
            for j in 0..(1 << i) {
                e[j] = FM::Target::from(0);
                e[j + (1 << i)] = e[j + (1 << i)].clone() * FM::Target::from(-1);
            }
            let mut e = FM::idft(FM::multiply(FM::dft(e), gdft));
            for j in 0..(1 << i) { e[j] = g[j].clone() }
            g.coef = e;
        }
        g.pre(n)
    }

    fn reciprocal(&mut self) {
        for i in 0..self.len() {
            let j = self.len() - 1 - i;
            if i < j {
                self.coef.swap(i, j);
            }
            else {
                break;
            }
        }
    }

    pub fn moduler(self, mut b: Self) -> Self {
        let mut a = self;
        let n = a.len();
        let m = b.len();
        if n < m {
            a
        }
        else {
            a.reciprocal();
            b.reciprocal();
            let nb = (n - m + 1).next_power_of_two();
            let mut q = (a.clone().pre(nb) * b.clone().pre(nb).inv2()).pre(n - m + 1);
            a.reciprocal();
            b.reciprocal();
            q.reciprocal();
            (a - b * q).pre(m)
        }
    }
}

impl<FM: FpsMultiply> std::ops::Index<usize> for FormalPowerSeries<FM> {
    type Output = FM::Target;
    fn index(&self, i: usize) -> &Self::Output { &self.coef[i] }
}

impl<FM: FpsMultiply> std::ops::IndexMut<usize> for FormalPowerSeries<FM> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output { &mut self.coef[i] }
}

impl<FM: FpsMultiply> Add for FormalPowerSeries<FM> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let n = std::cmp::max(self.len(), rhs.len());
        let mut next = self.pre(n);
        for i in 0..rhs.len() { next[i] = next[i] + rhs[i]; }
        next
    }
}

impl<FM: FpsMultiply> Sub for FormalPowerSeries<FM> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let n = std::cmp::max(self.len(), rhs.len());
        let mut next = self.pre(n);
        for i in 0..rhs.len() { next[i] = next[i] - rhs[i]; }
        next
    }
}

impl<FM: FpsMultiply> Mul for FormalPowerSeries<FM> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let m = self.len() + rhs.len() - 1;
        let n = m.next_power_of_two();
        Self::new_raw(FM::idft(FM::multiply(FM::dft(self.pre(n).coef), FM::dft(rhs.pre(n).coef)))).pre(m)
    }
}

impl<T: Copy, FM: FpsMultiply> Mul<T> for FormalPowerSeries<FM> where FM::Target: Mul<T, Output=FM::Target> {
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self {
        for i in 0..self.len() { self[i] = self[i] * rhs; }
        self
    }
}

impl<FM: FpsMultiply> Div for FormalPowerSeries<FM> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv2()
    }
}

impl<T: Copy, FM: FpsMultiply> Div<T> for FormalPowerSeries<FM> where FM::Target: Div<T, Output=FM::Target> {
    type Output = Self;
    fn div(mut self, rhs: T) -> Self {
        for i in 0..self.len() { self[i] = self[i] / rhs; }
        self
    }
}

#[test]
fn inv_test() {
    use crate::math::modint::*;
    use crate::math::convolution::number_theoretic_transform::NttMod976224257;
    use crate::math::fps_multiply::ntt_multiply::NttMultiply;
    type FM = NttMultiply<NttMod976224257>;
    type P = FormalPowerSeries<FM>;
    let p = P::new(&[ModInt::new(1), ModInt::newi64(-1)]).pre(16);
    assert_eq!(p.inv().coef.iter().map(|x| x.value()).collect::<Vec<_>>(), vec![1; 16]);
}

#[test]
fn inv2_test() {
    use crate::math::modint::*;
    use crate::math::convolution::number_theoretic_transform::NttMod976224257;
    use crate::math::fps_multiply::ntt_multiply::NttMultiply;
    type FM = NttMultiply<NttMod976224257>;
    type P = FormalPowerSeries<FM>;
    let p = P::new(&[ModInt::new(1), ModInt::newi64(-1)]).pre(16);
    assert_eq!(p.inv2().coef.iter().map(|x| x.value()).collect::<Vec<_>>(), vec![1; 16]);
}

#[test]
fn inv2_test2() {
    use crate::math::modint::*;
    use crate::math::convolution::number_theoretic_transform::NttMod998244353;
    use crate::math::fps_multiply::ntt_multiply::NttMultiply;
    type FM = NttMultiply<NttMod998244353>;
    type P = FormalPowerSeries<FM>;
    let p = P::new(&[ModInt::new(5), ModInt::newi64(4), ModInt::newi64(3), ModInt::newi64(2), ModInt::newi64(1)]).pre(16);
    println!("{:?}", p.inv2().coef.into_iter().map(|x| x.value()).collect::<Vec<_>>());
}

