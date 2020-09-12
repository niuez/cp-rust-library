use crate::algebra::Field;
use crate::math::matrix::matrix2d::Matrix2D;
use crate::math::matrix::sparse_matrix::SparseMatrix;
use crate::math::matrix::Matrix;

pub trait KrylovGen: Matrix {
    fn generate_krylov_sequence(&self, b: &[Self::Elem], len: usize) -> Vec<Vec<Self::Elem>>;
}

impl<F: Field> KrylovGen for Matrix2D<F> {
    fn generate_krylov_sequence(&self, b: &[F], len: usize) -> Vec<Vec<F>> {
        let mut v = Vec::with_capacity(len);
        v.push(b.to_vec());
        let n = self.height();
        for i in 1..len {
            let mut sum = vec![F::zero(); n];
            let ab = &v[i - 1];
            for j in 0..n {
                let aj = &self[j];
                for k in 0..n {
                    sum[j] += aj[k] * ab[k];
                }
            }
            v.push(sum);
        }
        v
    }
}

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

