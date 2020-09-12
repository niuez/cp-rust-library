use crate::algebra::Field;
use crate::math::matrix::Matrix;
use crate::math::matrix::diagonal_matrix::DiagonalMatrix;

use std::ops::Mul;

#[derive(Clone)]
pub struct SparseMatrix<F: Field> {
    row: Box<[Box<[(usize, F)]>]>,
    h: usize,
    w: usize,
}

impl<F: Field> Matrix for SparseMatrix<F> {
    type Elem = F;
    fn height(&self) -> usize { self.h }
    fn width(&self) -> usize { self.w }
}

impl<F: Field> SparseMatrix<F> {
    pub fn init_uncheck<I: IntoIterator<Item=(usize, usize, F)>>(elems: I, h: usize, w: usize) -> Self {
        let mut row = vec![Vec::new(); h];
        for (i, j, v) in elems.into_iter() {
            row[i].push((j, v));
        }
        let row = row.into_iter().map(|v| v.into_boxed_slice()).collect::<Vec<_>>().into_boxed_slice();
        Self { row, h, w }
    }
    pub fn row_iter<'a>(&'a self, i: usize) -> impl Iterator<Item=&'a (usize, F)> {
        self.row[i].iter()
    }
}

impl<F: Field> Mul<DiagonalMatrix<F>> for SparseMatrix<F> {
    type Output = SparseMatrix<F>;
    fn mul(mut self, rhs: DiagonalMatrix<F>) -> Self::Output {
        assert!(self.width() == rhs.height());
        self.row.iter_mut().for_each(|r| r.iter_mut().for_each(|(i, v)| *v *= rhs.val(*i)));
        self
    }
}
