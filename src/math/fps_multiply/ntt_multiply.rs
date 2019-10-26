use crate::math::fps_multiply::*;
use crate::math::modint::*;
use crate::math::convolution::number_theoretic_transform::*;

pub struct NttMultiply<N: NttMod>(std::marker::PhantomData<N>);

impl<N: NttMod> FpsMultiply for NttMultiply<N> {
    type Target = ModInt<N>;
    type Output = ModInt<N>;
    fn dft(arr: &[Self::Target]) -> Vec<Self::Output> { number_theoretic_transform(arr) }
    fn idft(arr: &[Self::Output]) -> Vec<Self::Target> { inverse_number_theoretic_transform(arr) }
    fn multiply(mut a: Vec<Self::Output>, b: Vec<Self::Output>) -> Vec<Self::Output> {
        for i in 0..a.len() { a[i] *= b[i] }
        a
    }
}
