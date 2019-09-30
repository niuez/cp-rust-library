use crate::algebra::*;

pub fn fast_mobius_transform<M: Monoid + Inv>(a: &[M], upper: bool) -> Vec<M> {
    let n = a.len();
    let mut i = 1;
    let mut a = a.to_vec();
    while i < n {
        for j in 0..n {
            if (j & i) == 0 {
                if upper { a[j] = a[j].op(&a[j | i].inv()); }
                else     { a[j | i] = a[j | i].op(&a[j].inv());  }
            }
        }
        i <<= 1;
    }
    a
}

#[cfg(test)]
mod test_fmt {
    use crate::algebra::*;
    use super::*;
    #[derive(Clone, PartialEq, Eq, Debug)]
    struct Am(i64);
    impl Magma for Am { fn op(&self, rhs: &Self) -> Self { Am(self.0 + rhs.0) } }
    impl Unital for Am { fn identity() -> Self { Am(0) } }
    impl Associative for Am {}
    impl Inv for Am { fn inv(&self) -> Self { Am(-self.0) } }

    #[test]
    fn test_fmt() {
        let a = vec![Am(1), Am(10), Am(100), Am(1000)];
        let b = vec![Am(1), Am(11), Am(101), Am(1111)];
        assert_eq!(fast_mobius_transform(&b, false), a);
    }
}
