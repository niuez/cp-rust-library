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

#[test]
fn matrix_pow_test() {
    use crate::math::modint::*;
    use crate::math::convolution::number_theoretic_transform::NttMod998244353;
    use crate::math::fps_multiply::ntt_multiply::NttMultiply;
    use crate::random::Xorshift128;
    use crate::math::fast_kitamasa::fast_kitamasa;
    type M = ModInt<NttMod998244353>;
    type FM = NttMultiply<NttMod998244353>;
    let n = 1000;
    let mut a = Matrix2D::zero(4, 4);
    for i in 0..4 {
        for j in 0..4 {
            a[i][j] = M::new((i * 4 + j) as u32);
        }
    }
    let mut rng = Xorshift128::new(91);
    let c = find_minimal_polynomial_from_matrix_pow(&mut rng, &a);
    println!("c = {:?}", c);
    let mut b = fast_kitamasa::<FM>(&c, n);
    println!("b = {:?}", b);
    b.reverse();

    let mut ans = Matrix2D::zero(4, 4);
    {
        let mut now = Matrix2D::zero(4, 4);
        for i in 0..4 {
            now[i][i] = M::new(1);
        }
        for i in 0..b.len() {
            ans = ans + now.clone() * b[i];
            now = now * a.clone();
        }
        println!("{:?}", ans);
    }

    let mut res = Matrix2D::zero(4, 4);
    {
        for i in 0..4 {
            res[i][i] = M::new(1);
        }
        for _ in 0..n {
            res = res * a.clone();
        }
        println!("{:?}", res);
    }
}
