use crate::math::fps_multiply::*;
use crate::math::modint::*;
use crate::math::convolution::number_theoretic_transform::*;

pub struct NttMultiply<N: NttMod>(std::marker::PhantomData<N>);

impl<N: NttMod> FpsMultiply for NttMultiply<N> {
    type Target = ModInt<N>;
    type Output = Vec<ModInt<N>>;
    fn dft(arr: Vec<Self::Target>) -> Self::Output { number_theoretic_transform(arr) }
    fn idft(arr: Self::Output) -> Vec<Self::Target> { inverse_number_theoretic_transform(arr) }
    fn multiply(mut a: Self::Output, b: Self::Output) -> Self::Output {
        for i in 0..a.len() { a[i] *= b[i] }
        a
    }
    fn multiply_self(mut a: Self::Output) -> Self::Output {
        for i in 0..a.len() { a[i] = a[i] * a[i] }
        a
    }
}
