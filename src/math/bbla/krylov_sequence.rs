use crate::algebra::Field;
use crate::math::matrix::matrix2d::Matrix2D;
use crate::math::matrix::sparse_matrix::SparseMatrix;
use crate::math::matrix::Matrix;

pub trait KrylovGen: Matrix {
    type Iter: IntoIterator<Item = Vec<Self::Elem>>;
    fn generate_krylov_sequence(self, b: Vec<Self::Elem>) -> Self::Iter;
}

pub struct KrylovIterMatrix2D<F: Field> {
    a: Matrix2D<F>,
    b: Vec<F>,
    n: usize,
}

impl<'a, F: Field> std::iter::Iterator for KrylovIterMatrix2D<F> {
    type Item = Vec<F>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = vec![F::zero(); self.n];
        for j in 0..self.n {
            let aj = &self.a[j];
            for k in 0..self.n {
                sum[j] += aj[k] * self.b[k];
            }
        }
        Some(std::mem::replace(&mut self.b, sum))
    }
}

impl<F: Field> KrylovGen for Matrix2D<F> {
    type Iter = KrylovIterMatrix2D<F>;
    fn generate_krylov_sequence(self, b: Vec<F>) -> Self::Iter {
        let n = self.height();
        KrylovIterMatrix2D { a: self, b, n }
    }
}

pub struct KrylovIterSparseMatrix<F: Field> {
    a: SparseMatrix<F>,
    b: Vec<F>,
    n: usize,
}

impl<'a, F: Field> std::iter::Iterator for KrylovIterSparseMatrix<F> {
    type Item = Vec<F>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = vec![F::zero(); self.n];
        for i in 0..self.n {
            for (j, v) in self.a.row_iter(i) {
                sum[i] += *v * self.b[*j];
            }
        }
        Some(std::mem::replace(&mut self.b, sum))
    }
}

impl<F: Field> KrylovGen for SparseMatrix<F> {
    type Iter = KrylovIterSparseMatrix<F>;
    fn generate_krylov_sequence(self, b: Vec<F>) -> Self::Iter {
        let n = self.height();
        KrylovIterSparseMatrix { a: self, b, n }
    }
}


/*
impl<F: Field> KrylovGen for SparseMatrix<F> {
    fn generate_krylov_sequence(&self, b: &[F], len: usize) -> Vec<Vec<F>> {
        let mut v = Vec::with_capacity(len);
        v.push(b.to_vec());
        let n = self.height();
        for s in 1..len {
            let mut sum = vec![F::zero(); n];
            let ab = &v[s - 1];
            for i in 0..n {
                for (j, v) in self.row_iter(i) {
                    sum[i] += *v * ab[*j];
                }
            }
            v.push(sum);
        }
        v
    }
}
*/
