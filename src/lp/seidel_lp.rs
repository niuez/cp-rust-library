use std::cmp::Ordering;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Xorshift {
    seed: u64,
}
impl Xorshift {
    #[allow(dead_code)]
    pub fn new() -> Xorshift {
        Xorshift {
            seed: 0xf0fb588ca2196dac,
        }
    }
    #[allow(dead_code)]
    pub fn with_seed(seed: u64) -> Xorshift {
        Xorshift { seed: seed }
    }
    #[inline]
    #[allow(dead_code)]
    pub fn next(&mut self) -> u64 {
        self.seed = self.seed ^ (self.seed << 13);
        self.seed = self.seed ^ (self.seed >> 7);
        self.seed = self.seed ^ (self.seed << 17);
        self.seed
    }
    #[inline]
    #[allow(dead_code)]
    pub fn rand(&mut self, m: u64) -> u64 {
        self.next() % m
    }
    #[inline]
    #[allow(dead_code)]
    // 0.0 ~ 1.0
    pub fn randf(&mut self) -> f64 {
        use std::mem;
        const UPPER_MASK: u64 = 0x3FF0000000000000;
        const LOWER_MASK: u64 = 0xFFFFFFFFFFFFF;
        let tmp = UPPER_MASK | (self.next() & LOWER_MASK);
        let result: f64 = unsafe { mem::transmute(tmp) };
        result - 1.0
    }
}


#[derive(Debug)]
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

pub fn seidel_lp(xrsh: &mut Xorshift, d: usize, c: &[f64], mat: &[(Vec<f64>, Param<f64>)]) -> Option<Vec<Param<f64>>> {
    match d {
        0 => unreachable!("0 dimension"),
        1 => {
            assert!(c.len() == 1, "length of c must be 1 at this point");
            let (h, l, z) = mat
                .iter()
                .fold((Param(0f64, 1f64), Param(0f64, -1f64), Param(0f64, 0f64)), |(h, l, z), (a, Param(x, y))| {
                    assert!(a.len() == 1, "length of a must be 1 at this point");
                    println!("{:?}", (a, Param(x, y)));
                    match a[0].partial_cmp(&0f64) {
                        None => unreachable!("a[0] is Nan?"),
                        Some(Ordering::Greater) => (std::cmp::min(h, Param(x / a[0], 0f64)), l, z),
                        Some(Ordering::Less) => (h, std::cmp::max(l, Param(x / a[0], 0f64)), z),
                        Some(Ordering::Equal) => (h, l, std::cmp::min(z, Param(*x, *y))),
                    }
                });
            println!("{:?}", (h, l, z));
            println!("c = {:?}\n", c);
            if z != Param(0f64, 0f64) || h < l {
                None
            }
            else if c[0] >= 0f64 { Some(vec![h]) }
            else { Some(vec![l]) }
        }
        d if mat.is_empty() => {
            Some((0..d).map(|i| Param(0f64, if c[i] >= 0f64 { 1f64 } else { -1f64 })).collect())
        }
        d => {
            let rmi = (xrsh.next() as usize) % mat.len();
            println!("d = {} rmi = {}", d, rmi);
            let (a, ap) = mat[rmi].clone();
            let next_mat = (0..mat.len()).filter(|i| *i != rmi).map(|i| mat[i].clone()).collect::<Vec<_>>();
            let v = if let Some(v) = seidel_lp(xrsh, d, c, &next_mat) { v } else { return None };
            let value = (0..d)
                .map(|i| Param(a[i] * v[i].0, a[i] * v[i].1))
                .fold(Param(0f64, 0f64), |Param(x1, y1), Param(x2, y2)| {
                    Param(x1 + x2, y1 + y2)
                }) ;
            println!("d = {} rmi = {} value = {:?} ap = {:?}", d, rmi, value, ap);
            println!("v = {:?}", v);
            println!("a[k] = {:?}", a);
            if  value <= ap {
                    Some(v)
                }
            else {
                let k = if let Some(k) = (0..d).rev().find(|i| a[*i] != 0f64) { k } else { return None };
                println!("k = {}", k);
                let mut bar_mat = next_mat.iter().map(|(b, Param(x, y))| {
                    let ratio = b[k] / a[k];
                    let v = (0..d)
                        .filter(|i| *i != k)
                        .map(|i| {
                            b[i] - ratio * a[i]
                        })
                        .collect::<Vec<_>>();
                    (v, Param(x - ratio * ap.0, y - ratio * ap.1))
                }).collect::<Vec<_>>();
                let bar_c = (0..d)
                    .filter(|i| *i != k)
                    .map(|i| {
                        c[i] - (c[k] / a[k]) * a[i]
                    }).collect::<Vec<_>>();

                let f = (0..d).filter(|i| *i != k).map(|i| {
                    - (1f64 / a[k]) * a[i]
                }).collect::<Vec<_>>();
                let fp = Param(0f64 - (1f64 / a[k]) * ap.0, 1f64 - (1f64 / a[k]) * ap.1);
                let g = (0..d).filter(|i| *i != k).map(|i| {
                    - (1f64 / a[k]) * a[i]
                }).collect::<Vec<_>>();
                let gp = Param(0f64 - (1f64 / a[k]) * ap.0, -1f64 - (1f64 / a[k]) * ap.1);
                bar_mat.push((f, fp));
                bar_mat.push((g, gp));

                let mut v = if let Some(v) = seidel_lp(xrsh, d - 1, &bar_c, &bar_mat) { v } else { return None; };

                v.insert(k, Param(0f64, 0f64));
                let Param(su, sw) = (0..d)
                    .map(|i| Param(a[i] * v[i].0, a[i] * v[i].1))
                    .fold(Param(0f64, 0f64), |Param(x1, y1), Param(x2, y2)| {
                        Param(x1 + x2, y1 + y2)
                    });
                println!("a = {:?} ap = {:?}, {:?}, {:?}", a, ap, su, sw);
                v[k] = Param((ap.0 - su) / a[k], (ap.1 - sw) / a[k]);
                Some(v)
            }
        }
    }
}

#[test]
fn lp_test() {
    let mut xrsh = Xorshift::new();
    let ans = seidel_lp(&mut xrsh, 2, &[1f64, 1f64], &[(vec![3f64, 1f64], Param(9f64, 0f64)), (vec![1f64, 3f64], Param(6f64, 0f64))]);
    println!("{:?}\n", ans);
}
