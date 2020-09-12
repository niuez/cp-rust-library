pub mod matrix2d;
pub mod diagonal_matrix;
pub mod sparse_matrix;

use crate::algebra::Field;

pub trait Matrix {
    type Elem: Field;
    fn height(&self) -> usize;
    fn width(&self) -> usize;
}
