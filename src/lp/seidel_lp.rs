use std::cmp::Ordering;

pub struct Param<T>(T, T);

impl<T> Clone for Param<T>
where T: Clone {
    fn clone(&self) -> Self {
        Param(self.0.clone(), self.1.clone())
    }
}
impl<T> Copy for Param<T> where T: Copy {}

impl<T> PartialEq for Param<T>
where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl<T> Eq for Param<T> where T: PartialEq {}

impl<T> Ord for Param<T>
where T: PartialOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl<T> PartialOrd for Param<T>
where T: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.1.partial_cmp(&other.1) {
            Some(Ordering::Equal) => self.0.partial_cmp(&other.0),
            o => o,
        }
    }
}

pub fn seidel_lp(d: usize, c: &[f64], mat: &[(Vec<f64>, Param<f64>)]) -> Option<Vec<Param<f64>>> {
    match d {
        0 => unreachable!("0 dimension"),
        1 => {
            assert!(c.len() == 1, "length of c must be 1 at this point");
            let (h, l, z) = mat
                .iter()
                .fold((Param(0f64, 1f64), Param(0f64, -1f64), Param(0f64, 0f64)), |(h, l, z), (a, Param(x, y))| {
                    assert!(a.len() == 1, "length of a must be 1 at this point");
                        match a[0].partial_cmp(&0f64) {
                            None => unreachable!("a[0] is Nan?"),
                            Some(Ordering::Greater) => (std::cmp::min(h, Param(x / a[0], y / a[0])), l, z),
                            Some(Ordering::Less) => (h, std::cmp::min(l, Param(x / a[0], y / a[0])), z),
                            Some(Ordering::Equal) => (h, l, std::cmp::min(z, Param(*x, *y))),
                        }
                });
            if z != Param(0f64, 0f64) || h < l {
                None
            }
            else if c[0] >= 0f64 { Some(vec![h]) }
            else { Some(vec![l]) }
        }
        d => {
            unimplemented!();
        }
    }
}
