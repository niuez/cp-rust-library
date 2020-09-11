use crate::algebra::Field;
use crate::math::matrix::matrix2d::Matrix2D;

pub trait KrylovGen<F: Field> {
    fn generate_krylov_sequence(&self, b: &[F], len: usize) -> Vec<Vec<F>>;
}

impl<F: Field> KrylovGen<F> for Matrix2D<F> {
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
