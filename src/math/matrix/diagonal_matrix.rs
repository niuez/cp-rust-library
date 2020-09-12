use crate::algebra::Field;
use crate::math::matrix::Matrix;
use crate::math::matrix::matrix2d::Matrix2D;

use std::ops::{ Add, Sub, Mul };

#[derive(Clone)]
pub struct DiagonalMatrix<F: Field> {
    c: Box<[F]>,
    n: usize,
}

impl<F: Field> Matrix for DiagonalMatrix<F> {
    type Elem = F;
    fn height(&self) -> usize { self.n }
    fn width(&self) -> usize { self.n }
}

impl<F: Field> DiagonalMatrix<F> {
    pub fn init(c: &[F]) -> Self {
        let n = c.len();
        Self { c: c.to_vec().into_boxed_slice(), n }
    }
    pub fn determinant(&self) -> F {
        self.c.iter().fold(F::one(), |x, &y| x * y)
    }
}

impl<F: Field> Add<Self> for DiagonalMatrix<F> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        assert!(self.n == rhs.n);
        for i in 0..self.n {
            self.c[i] += rhs.c[i];
        }
        self
    }
}

impl<F: Field> Sub<Self> for DiagonalMatrix<F> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        assert!(self.n == rhs.n);
        for i in 0..self.n {
            self.c[i] -= rhs.c[i];
        }
        self
    }
}

impl<F: Field> Mul<Self> for DiagonalMatrix<F> {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        assert!(self.n == rhs.n);
        for i in 0..self.n {
            self.c[i] *= rhs.c[i];
        }
        self
    }
}

impl<F: Field> Add<Matrix2D<F>> for DiagonalMatrix<F> {
    type Output = Matrix2D<F>;
    fn add(self, mut rhs: Matrix2D<F>) -> Self::Output {
        assert!(self.n == rhs.height());
        assert!(self.n == rhs.width());
        for i in 0..self.n {
            rhs[i][i] += self.c[i];
        }
        rhs
    }
}

impl<F: Field> Sub<Matrix2D<F>> for DiagonalMatrix<F> {
    type Output = Matrix2D<F>;
    fn sub(self, mut rhs: Matrix2D<F>) -> Self::Output {
        assert!(self.n == rhs.height());
        assert!(self.n == rhs.width());
        for i in 0..self.n {
            rhs[i][i] = self.c[i] - rhs[i][i];
        }
        rhs
    }
}

impl<F: Field> Mul<Matrix2D<F>> for DiagonalMatrix<F> {
    type Output = Matrix2D<F>;
    fn mul(self, mut rhs: Matrix2D<F>) -> Self::Output {
        assert!(self.n == rhs.height());
        for i in 0..self.n {
            for j in 0..rhs.width() {
                rhs[i][j] *= self.c[i];
            }
        }
        rhs
    }
}

impl<F: Field> Mul<DiagonalMatrix<F>> for Matrix2D<F> {
    type Output = Matrix2D<F>;
    fn mul(mut self, rhs: DiagonalMatrix<F>) -> Self::Output {
        assert!(self.width() == rhs.n);
        for j in 0..self.width() {
            for i in 0..self.height() {
                self[i][j] *= rhs.c[j]
            }
        }
        self
    }
}
