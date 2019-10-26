use crate::math::fps_multiply::*;
use crate::math::modint::*;
use crate::math::complex::Complex;
use crate::math::convolution::fast_fourier_transform::*;

pub struct FftModMultiply<M: Mod>(std::marker::PhantomData<M>);

impl<M: Mod> FpsMultiply for FftModMultiply<M> {
    type Target = ModInt<M>;
    type Output = Complex;
    fn dft(arr: &[Self::Target]) -> Vec<Self::Output> { fast_fourier_transform(&arr.iter().map(|&x| Complex::new(x.value() as f64, 0f64)).collect::<Vec<_>>()) }
    fn idft(arr: &[Self::Output]) -> Vec<Self::Target> { inverse_fast_fourier_transform(arr).into_iter().map(|z| ModInt::from(z.x.round() as i64)).collect() }
    fn multiply(mut a: Vec<Self::Output>, b: Vec<Self::Output>) -> Vec<Self::Output> {
        for i in 0..a.len() { a[i] *= b[i] }
        a
    }
}
