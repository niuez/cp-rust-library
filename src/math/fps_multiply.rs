pub mod ntt_multiply;
pub mod fft_multiply;

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
