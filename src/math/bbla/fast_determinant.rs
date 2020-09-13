use crate::algebra::Field;
use crate::random::{ Random, RandomGen };
use crate::math::matrix::Matrix;
use crate::math::matrix::matrix2d::Matrix2D;
use crate::math::matrix::sparse_matrix::SparseMatrix;
use crate::math::matrix::diagonal_matrix::DiagonalMatrix;
use crate::math::bbla::minimal_polynomial::find_minimal_polynomial_from_matrix_pow;

pub fn fast_determinant<F: Field + RandomGen + std::fmt::Debug, R: Random>(rng: &mut R, a: Matrix2D<F>) -> F {
    assert!(a.height() == a.width());
    let n = a.height();
    let mut d = vec![F::zero(); n];
    {
        let mut i = 0;
        while i < n {
            d[i] = F::rand_gen(rng);
            if d[i] != F::zero() {
                i += 1;
            }
        }
    }
    let d = DiagonalMatrix::init(&d);
    let d_det = d.determinant();
    let ad = a * d;
    let c = find_minimal_polynomial_from_matrix_pow(rng, &ad);
    let det = c[c.len() - 1] / c[0];
    let det = if n % 2 == 0 { det } else { -det };
    return det / d_det
}

pub fn fast_determinant_sparse<F: Field + RandomGen + std::fmt::Debug, R: Random>(rng: &mut R, a: SparseMatrix<F>) -> F {
    assert!(a.height() == a.width());
    let n = a.height();
    let mut d = vec![F::zero(); n];
    {
        let mut i = 0;
        while i < n {
            d[i] = F::rand_gen(rng);
            if d[i] != F::zero() {
                i += 1;
            }
        }
    }
    let d = DiagonalMatrix::init(&d);
    let d_det = d.determinant();
    let ad = a * d;
    let c = find_minimal_polynomial_from_matrix_pow(rng, &ad);
    let det = c[c.len() - 1] / c[0];
    let det = if n % 2 == 0 { det } else { -det };
    return det / d_det
}
