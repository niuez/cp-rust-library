pub mod knuth_yao_speedup;

use std::ops::Range;

pub trait RangeFunction {
    type Output: Copy + Ord;
    fn len(&self) -> usize;
    fn func(&self, ran: Range<usize>) -> Self::Output;
}

/// Monge = Quadrangle Inequality
/// 
/// f(a | b) + f(a & b) >= f(a) + f(b)
pub trait Monge: RangeFunction {}

/// Monotone
///
/// i < j ==> argmin(i, *) <= argmin(j, *)
/// 行ごとの最小値の配置が右下に単調に下がっていく.
pub trait Monotone {}

/// TotallyMonotone
///
/// 任意の部分行列がmonotone
pub trait TotallyMonotone: Monotone {}

impl<M: Monge> Monotone for M {}
impl<M: Monge> TotallyMonotone for M {}
