pub mod monotone_cht;

use std::ops::{ Add, Sub, Mul };

pub trait LineDecimal: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Copy + PartialOrd {}

impl LineDecimal for i64 {}
impl LineDecimal for f64 {}

pub struct Line<T: LineDecimal> { pub a: T, pub b: T, }

impl<T: LineDecimal> Line<T> {
    pub fn new(a: T, b: T) -> Self { Line { a, b } }
    pub fn get(&self, x: T) -> T { self.a * x + self.b }
}

impl<T: LineDecimal> Clone for Line<T> {
    fn clone(&self) -> Self { Line { a: self.a, b: self.b } }
}
