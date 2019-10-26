use crate::math::modint::*;
use crate::math::convolution::numeric_theoretic_transform::*;

use std::ops::{ Add, Sub, Mul, Div };

pub trait BasicOpe: Sized + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Copy + From<i64> {}
impl<T: Sized + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Copy + From<i64>> BasicOpe for T {}

pub trait FpsMultiply: Sized {
    type Target: BasicOpe;
    type Output: Clone;
    fn dft(arr: &[Self::Target]) -> Vec<Self::Output>;
    fn idft(arr: &[Self::Output]) -> Vec<Self::Target>;
    fn multiply(a: Vec<Self::Output>, b: Vec<Self::Output>) -> Vec<Self::Output>;
}

pub struct FormalPowerSeries<T: BasicOpe, FM: FpsMultiply<Target=T>> {
    coef: Vec<T>,
    _p: std::marker::PhantomData<FM>,
}

impl<T: BasicOpe, FM: FpsMultiply<Target=T>> Clone for FormalPowerSeries<T, FM> {
    fn clone(&self) -> Self { FormalPowerSeries::new_raw(self.coef.clone()) }
}

impl<T: BasicOpe, FM: FpsMultiply<Target=T>> FormalPowerSeries<T, FM> {
    fn new_raw(coef: Vec<T>) -> Self { FormalPowerSeries { coef: coef, _p: std::marker::PhantomData } }
    pub fn new(coef: &[T]) -> Self {
        let mut coef = coef.to_vec();
        let n = (0usize.count_zeros()
                 - coef.len().leading_zeros()
                 - if coef.len().count_ones() == 1 { 1 } else { 0 }
                 ) as usize;
        coef.resize(1 << n, T::from(0i64));
        FormalPowerSeries::new_raw(coef)
    }
    pub fn len(&self) -> usize { self.coef.len() }
    pub fn pre(mut self, d: usize) -> Self {
        self.coef.resize(d, T::from(0i64));
        self
    }

    pub fn inv(&self) -> Self {
        let mut g = FormalPowerSeries::new(&[T::from(1) / self[0]]);
        for i in 0..self.len().trailing_zeros() {
            g = (g.clone() * T::from(2i64) - g.clone() * g * self.clone().pre(1 << (i + 1))).pre(1 << (i + 1));
        }
        g.pre(self.len())
    }

    pub fn inv2(&self) -> Self {
        let mut g = FormalPowerSeries::new(&[T::from(1) / self[0]]);
        let n = self.len();
        for i in 0..self.len().trailing_zeros() {
            /*
            g = g.pre(1 << (i + 1));
            let mut ft = numeric_theoretic_transform(&self.clone().pre(1 << (i + 1)).coef);
            let gt = numeric_theoretic_transform(&g.coef);
            for j in 0..(1 << (i + 1)) { ft[j] *= gt[j]; }
            let mut e = inverse_numeric_theoretic_transform(&ft);
            */
            g = g.pre(1 << (i + 1));
            let gdft = FM::dft(&g.coef);
            let mut e = FM::idft(&FM::multiply(FM::dft(&self.clone().pre(1 << (i + 1)).coef), gdft.clone()));
            for j in 0..(1 << i) {
                e[j] = T::from(0i64);
                e[j + (1 << i)] = e[j + (1 << i)].clone() * T::from(-1i64);
            }
            let mut e = FM::idft(&FM::multiply(FM::dft(&e), gdft));
            for j in 0..(1 << i) { e[j] = g[j].clone() }
            g.coef = e;
        }
        g.pre(n)
    }
}

impl<T: BasicOpe, FM: FpsMultiply<Target=T>> std::ops::Index<usize> for FormalPowerSeries<T, FM> {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output { &self.coef[i] }
}

impl<T: BasicOpe, FM: FpsMultiply<Target=T>> std::ops::IndexMut<usize> for FormalPowerSeries<T, FM> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output { &mut self.coef[i] }
}

impl<T: BasicOpe, FM: FpsMultiply<Target=T>> Add for FormalPowerSeries<T, FM> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let n = std::cmp::max(self.len(), rhs.len());
        let mut next = self.pre(n);
        for i in 0..rhs.len() { next[i] = next[i] + rhs[i]; }
        next
    }
}

impl<T: BasicOpe, FM: FpsMultiply<Target=T>> Sub for FormalPowerSeries<T, FM> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let n = std::cmp::max(self.len(), rhs.len());
        let mut next = self.pre(n);
        for i in 0..rhs.len() { next[i] = next[i] - rhs[i]; }
        next
    }
}

impl<T: BasicOpe, FM: FpsMultiply<Target=T>> Mul for FormalPowerSeries<T, FM> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let n = std::cmp::max(self.len(), rhs.len()) << 1;
        Self::new_raw(FM::idft(&FM::multiply(FM::dft(&self.pre(n).coef), FM::dft(&rhs.pre(n).coef))))
    }
}

impl<T: BasicOpe, FM: FpsMultiply<Target=T>> Mul<T> for FormalPowerSeries<T, FM> {
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self {
        for i in 0..self.len() { self[i] = self[i] * rhs; }
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
