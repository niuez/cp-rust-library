use crate::algebra::Field;


// a[i] = sum{j = 1 to d} c[j] * a[i - j] for d <= i < N
pub fn berlekamp_massey<F: Field>(s: &[F]) -> Vec<F> {
    let n = s.len();
    let mut b = vec![F::one()];
    let mut c = vec![F::one()];
    let mut y = F::one();
    let mut shift = 0;
    for len in 0..n {
        shift += 1;
        let x = (0..c.len())
            .map(|i| c[i] * s[len - i])
            .fold(F::zero(), |x, y| x + y);
        if x == F::zero() { continue }
        let old_c = c.clone();
        let freq = x / y;
        c.resize(std::cmp::max(c.len(), b.len() + shift), F::zero());
        for i in 0..b.len() {
            c[i + shift] -= freq * b[i];
        }
        if old_c.len() < c.len() {
            b = old_c;
            y = x;
            shift = 0;
        }
    }
    c.into_iter().skip(1).map(|x| -x).collect()
}
