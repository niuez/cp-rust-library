use crate::random::{ Random, RandomGen };
use crate::algebra::Field;
use crate::math::matrix::Matrix2D;
use crate::math::berlekamp_massey::berlekamp_massey;

pub fn find_minimal_polynomial<F: Field>(a: &[F]) -> Vec<F> {
    let c = berlekamp_massey(a);
    std::iter::once(-F::one()).chain(c.into_iter()).collect()
}

pub fn find_minimal_polynomial_from_vector<AV: AsRef<[V]>, V: AsRef<[F]>, F: Field + RandomGen, R: Random>(rng: &mut R, a: AV) -> Vec<F> {
    let m = a.as_ref()[0].as_ref().len();
    let u: Vec<_> = (0..m).map(|_| F::rand_gen(rng)).collect();
    let b: Vec<_> = a.as_ref().iter().map(|x| {
        x.as_ref().iter()
            .zip(u.iter())
            .map(|(x, y)| *x * *y)
            .fold(F::zero(), |x, y| x + y)
    }).collect();
    let c = find_minimal_polynomial(&b);
    c
}

pub fn find_minimal_polynomial_from_matrix_pow<F: Field + RandomGen, R: Random>(rng: &mut R, a: &Matrix2D<F>) -> Vec<F> {
    assert!(a.height() == a.weight());
    let n = a.height();
    let b: Vec<_> = (0..n).map(|_| F::rand_gen(rng)).collect();
    let mut v = vec![b];
    for i in 1..n*2 {
        let mut sum = vec![F::zero(); n];
        let ab = &v[i - 1];
        for j in 0..n {
            let aj = &a[j];
            for k in 0..n {
                sum[j] += aj[k] * ab[k];
            }
        }
        v.push(sum);
    }
    find_minimal_polynomial_from_vector(rng, &v)
}
