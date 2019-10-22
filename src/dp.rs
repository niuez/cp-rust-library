pub mod knuth_yao_speedup;

use std::ops::Range;

pub trait RangeFunction {
    type Output: Copy;
    fn len(&self) -> usize;
    fn func(&self, ran: Range<usize>) -> Self::Output;
}

/// Quadrangle Inequality
/// 
/// f(a | b) + f(a & b) >= f(a) + f(b)
pub trait QuadrangleInequality: RangeFunction {}
