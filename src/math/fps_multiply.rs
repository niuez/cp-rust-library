pub mod ntt_multiply;
pub mod ntt_arb_multiply;

use crate::algebra::Field;

pub trait FpsMultiply: Sized {
    type Target: Field;
    type Output: Clone;
    fn dft(arr: Vec<Self::Target>) -> Self::Output;
    fn idft(arr: Self::Output) -> Vec<Self::Target>;
    fn multiply(a: Self::Output, b: Self::Output) -> Self::Output;
    fn multiply_self(a: Self::Output) -> Self::Output;
}
