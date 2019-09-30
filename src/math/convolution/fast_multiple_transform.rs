use crate::algebra::*;

pub fn fast_multiple_transform<M: Monoid>(a: &[M]) -> Vec<M> {
    let n = a.len();
    let mut a = a.to_vec();
    let mut sieve = vec![true; n];
    for p in 2..n {
        if sieve[p] {
            for k in (1..(n - 1) / p + 1).rev() {
                sieve[k * p] = false;
                a[k] = a[k].op(&a[k * p]);
            }
        }
    }
    for i in 1..n {
        a[i] = a[i].op(&a[0]);
    }
    a
}

pub fn fast_inverse_multiple_transform<M: Monoid + Inv>(a: &[M]) -> Vec<M> {
    let n = a.len();
    let mut a = a.to_vec();
    let mut sieve = vec![true; n];
    for i in 1..n {
        a[i] = a[i].op(&a[0].inv());
    }
    for p in 2..n {
        if sieve[p] {
            for k in 1..(n - 1) / p + 1 {
                sieve[k * p] = false;
                a[k] = a[k].op(&a[k * p].inv());
            }
        }
    }
    a
}

#[cfg(test)]
mod test_fmmt {
    use crate::algebra::*;
    use super::*;
    #[derive(Clone, PartialEq, Eq, Debug)]
    struct Am(i64);
    impl Magma for Am { fn op(&self, rhs: &Self) -> Self { Am(self.0 + rhs.0) } }
    impl Unital for Am { fn identity() -> Self { Am(0) } }
    impl Associative for Am {}
    impl Inv for Am { fn inv(&self) -> Self { Am(-self.0) } }

    #[test]
    fn test_fmmt() {
        let a: Vec<_> = (0..12).map(|x| Am(x)).collect();
        let b = vec![Am(0), Am(66), Am(30), Am(18), Am(12), Am(15), Am(6), Am(7), Am(8), Am(9), Am(10), Am(11)];
        assert_eq!(fast_multiple_transform(&a), b);
        assert_eq!(fast_inverse_multiple_transform(&b), a);
    }
}
