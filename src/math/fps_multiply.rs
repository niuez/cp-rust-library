pub mod ntt_multiply;
pub mod ntt_arb_multiply;

use std::ops::{ Add, Sub, Mul, Div };

pub trait BasicOpe: Sized + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Copy + From<i64> {}
impl<T: Sized + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Copy + From<i64>> BasicOpe for T {}

pub trait FpsMultiply: Sized {
    type Target: BasicOpe;
    type Output: Clone;
    fn dft(arr: Vec<Self::Target>) -> Self::Output;
    fn idft(arr: Self::Output) -> Vec<Self::Target>;
    fn multiply(a: Self::Output, b: Self::Output) -> Self::Output;
}
