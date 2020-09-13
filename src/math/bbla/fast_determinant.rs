use crate::algebra::Field;
use crate::random::{ Random, RandomGen };
use crate::math::matrix::Matrix;
use crate::math::matrix::diagonal_matrix::DiagonalMatrix;
use crate::math::bbla::minimal_polynomial::find_minimal_polynomial_from_matrix_pow;
use crate::math::bbla::krylov_sequence::KrylovGen;

pub fn fast_determinant<F, R, M>(rng: &mut R, a: M) -> F
where
    F: Field + RandomGen,
    R: Random,
    M: Matrix<Elem=F> + std::ops::Mul<DiagonalMatrix<F>>,
    <M as std::ops::Mul<DiagonalMatrix<F>>>::Output: Matrix<Elem=F> + KrylovGen,
{
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
    let c = find_minimal_polynomial_from_matrix_pow(rng, ad);
    let det = c[c.len() - 1] / c[0];
    let det = if n % 2 == 0 { det } else { -det };
    return det / d_det
}
