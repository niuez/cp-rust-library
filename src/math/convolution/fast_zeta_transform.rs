use crate::algebra::*;

pub fn fast_zeta_transform<M: Monoid>(a: &[M], upper: bool) -> Vec<M> {
    let n = a.len();
    let mut i = 1;
    let mut a = a.to_vec();
    while i < n {
        for j in 0..n {
            if (j & i) == 0 {
                if upper { a[j] = a[j].op(&a[j | i]); }
                else     { a[j | i] = a[j | i].op(&a[j]);  }
            }
        }
        i <<= 1;
    }
    a
}

#[cfg(test)]
mod test_fzt {
    use crate::algebra::*;
    use super::*;
    #[derive(Clone, PartialEq, Eq, Debug)]
    struct Am(usize);
    impl Magma for Am { fn op(&self, rhs: &Self) -> Self { Am(self.0 + rhs.0) } }
    impl Unital for Am { fn identity() -> Self { Am(0) } }
    impl Associative for Am {}

    #[test]
    fn test_fzt() {
        let a = [Am(1), Am(10), Am(100), Am(1000)];
        assert_eq!(fast_zeta_transform(&a, false), vec![Am(1), Am(11), Am(101), Am(1111)]);
    }
}
