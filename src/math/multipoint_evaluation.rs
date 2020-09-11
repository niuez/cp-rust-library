use crate::math::formal_power_series::FormalPowerSeries;
use crate::math::fps_multiply::FpsMultiply;
use crate::algebra::Field;

type Fps<FM> = FormalPowerSeries<FM>;

pub struct MultipointEvaluation<FM: FpsMultiply> {
    n: usize,
    sub: Vec<Fps<FM>>,
    //_p: std::marker::PhantomData<FM>,
}

impl<FM: FpsMultiply> MultipointEvaluation<FM> {
    pub fn new(x: &[FM::Target]) -> Self {
        let n = x.len();
        assert!(n.is_power_of_two(), "n must be power of two");
        let zero = FM::Target::zero();
        let one = FM::Target::one();
        let mut sub = vec![Fps::new(&[zero.clone()]); n * 2 - 1];
        for i in 0..n {
            sub[i + n - 1] = Fps::new(&[zero - x[i], one]);
        }
        for i in (1..n-1).rev() {
            sub[i] = sub[(i << 1) + 1].clone() * sub[(i << 1) + 2].clone();
        }
        Self {
            n,
            sub,
        }
    }

    fn evaluate_rec(&self, f: Fps<FM>, k: usize, ans: &mut Vec<FM::Target>) {
        if k >= self.n - 1 {
            ans[k + 1 - self.n] = f[0].clone();
        }
        else {
            self.evaluate_rec(f.clone().moduler(self.sub[(k << 1) + 1].clone()), (k << 1) + 1, ans);
            self.evaluate_rec(f.moduler(self.sub[(k << 1) + 2].clone()), (k << 1) + 2, ans);
        }
    }

    pub fn evaluate(&self, f: Fps<FM>) -> Vec<FM::Target> {
        let mut ans = vec![FM::Target::zero(); self.n];
        self.evaluate_rec(f, 0, &mut ans);
        ans
    }
}

#[test]
fn evaluation_test() {
    use crate::math::modint::*;
    use crate::math::convolution::number_theoretic_transform::NttMod998244353;
    use crate::math::fps_multiply::ntt_multiply::NttMultiply;
    type FM = NttMultiply<NttMod998244353>;
    type P = FormalPowerSeries<FM>;
    let f = P::new(&[ModInt::newi64(4), ModInt::newi64(3), ModInt::newi64(2), ModInt::newi64(1)]);
    let multi = MultipointEvaluation::new(&[ModInt::new(1), ModInt::new(2), ModInt::new(3), ModInt::new(0)]);
    println!("{:?}", multi.evaluate(f));
}

