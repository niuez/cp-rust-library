pub mod monotone_cht;
pub mod li_chao_segment_tree;

use std::ops::{ Add, Mul };

pub trait LineNumber: Add<Output=Self> + Mul<Output=Self> + Copy + PartialOrd {}

impl LineNumber for i64 {}
impl LineNumber for f64 {}

pub struct Line<T> { pub a: T, pub b: T, }

impl<T: LineNumber> Line<T> {
    pub fn new(a: T, b: T) -> Self { Line { a, b } }
    pub fn get(&self, x: T) -> T { self.a * x + self.b }
}

impl<T: LineNumber> Clone for Line<T> {
    fn clone(&self) -> Self { Line { a: self.a, b: self.b } }
}
