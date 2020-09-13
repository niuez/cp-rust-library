use crate::random::{ Random, RandomGen };
use crate::algebra::Field;
use crate::math::berlekamp_massey::berlekamp_massey;
use crate::math::bbla::krylov_sequence::KrylovGen;

pub fn find_minimal_polynomial<F: Field>(a: &[F]) -> Vec<F> {
    let c = berlekamp_massey(a);
    std::iter::once(-F::one()).chain(c.into_iter()).collect()
}

pub fn find_minimal_polynomial_from_vector<AV, V, F, R>(rng: &mut R, a: AV, n: usize) -> Vec<F>
where
    AV: IntoIterator<Item=V>,
    V: AsRef<[F]>,
    F: Field + RandomGen,
    R: Random
{
    let u: Vec<_> = (0..n).map(|_| F::rand_gen(rng)).collect();
    let b: Vec<_> = a.into_iter().map(|x| {
        x.as_ref().iter()
            .zip(u.iter())
            .map(|(x, y)| *x * *y)
            .fold(F::zero(), |x, y| x + y)
    }).collect();
    find_minimal_polynomial(&b)
}

pub fn find_minimal_polynomial_from_matrix_pow<R: Random, M: KrylovGen>(rng: &mut R, a: M) -> Vec<M::Elem>
where M::Elem: RandomGen
{
    assert!(a.height() == a.width());
    let n = a.height();
    let b: Vec<_> = (0..n).map(|_| M::Elem::rand_gen(rng)).collect();
    let iter = a.generate_krylov_sequence(b).into_iter().take(n * 2);
    find_minimal_polynomial_from_vector(rng, iter, n)
}

#[test]
fn matrix_pow_test() {
    use crate::math::modint::*;
    use crate::math::convolution::number_theoretic_transform::NttMod998244353;
    use crate::math::fps_multiply::ntt_multiply::NttMultiply;
    use crate::random::Xorshift128;
    use crate::math::fast_kitamasa::fast_kitamasa;
    use crate::math::matrix::matrix2d::Matrix2D;
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
    let c = find_minimal_polynomial_from_matrix_pow(&mut rng, a.clone());
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
