use crate::algebra::Field;
use crate::math::fps_multiply::FpsMultiply;
use crate::math::formal_power_series::FormalPowerSeries;

// sum { i = 0 to k } c[i] * a[k - i] = 0, c[0] == -1
pub fn fast_kitamasa<FM: FpsMultiply>(c: &[FM::Target], n: usize) -> Vec<FM::Target> {
    let k = c.len() - 1;
    let c = FormalPowerSeries::<FM>::new(c);
    let ic = c.inv2().pre(k - 1);

    let b_len = k.next_power_of_two();
    let mut b = vec![FM::Target::zero(); b_len];
    b[k - 1] = FM::Target::one();
    let bit = n.next_power_of_two().trailing_zeros() as usize;
    for s in (0..bit + 1).rev() {
        b.resize(b.len() * 2, FM::Target::zero());
        let bt = FM::dft(b);
        let bt = FM::multiply_self(bt);
        let beta = FormalPowerSeries::<FM>::new_raw(FM::idft(bt));
        let q = (beta.clone().pre(k - 1) * ic.clone()).pre(k - 1);
        let cq = beta - c.clone() * q;
        b = vec![FM::Target::zero(); b_len];
        for i in (k - 1)..(2 * k - 1) {
            b[i - (k - 1)] = cq[i];
        }
        if ((n >> s) & 1) == 1 {
            let freq = b[0];
            for i in 0..k-1 {
                b[i] = b[i + 1] + freq * c[i + 1];
            }
            b[k - 1] = freq * c[k];
        }
    }
    b.into_iter().take(k).collect()
}

#[test]
fn kitamasa_test() {
    use crate::math::modint::*;
    use crate::math::convolution::number_theoretic_transform::NttMod998244353;
    use crate::math::fps_multiply::ntt_multiply::NttMultiply;
    type FM = NttMultiply<NttMod998244353>;

    let c = [-ModInt::new(1), ModInt::new(1), ModInt::new(2), ModInt::new(3), ModInt::new(4)];
    let k = c.len() - 1;
    let mut a = vec![ModInt::new(1), ModInt::new(1), ModInt::new(1), ModInt::new(1)];
    for i in 0..11 {
        let mut sum = ModInt::new(0);
        for j in 0..k {
            sum += a[j + i] * c[k - j];
        }
        a.push(sum);
    }
    let b = fast_kitamasa::<FM>(&c, 10);
    assert_eq!(a[10], (0..k).map(|j| a[j] * b[k - 1 - j]).fold(ModInt::new(0), |x, y| x + y));
}

