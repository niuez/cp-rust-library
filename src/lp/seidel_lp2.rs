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


pub fn seidel_lp(xrsh: &mut Xorshift, d: usize, c: &[f64], mat: &[(Vec<f64>, f64)], bounds: &[(f64, f64)]) -> Option<Vec<f64>> {
    match d {
        0 => unreachable!("0 dimension"),
        1 => {
            assert!(c.len() == 1, "length of c must be 1 at this point");
            let (low, high) = bounds[0];
            let (h, l, z) = mat
                .iter()
                .fold((high, low, 0f64), |(h, l, z), (a, p)| {
                    assert!(a.len() == 1, "length of a must be 1 at this point");
                    println!("{:?}", (a, p));
                    match a[0].partial_cmp(&0f64) {
                        None => unreachable!("a[0] is Nan?"),
                        Some(Ordering::Greater) => {
                            let nh = if p / a[0] <= h { p / a[0] } else { h };
                            (nh, l, z)
                        }
                        Some(Ordering::Less) => {
                            let nl = if p / a[0] >= l { p / a[0] } else { l };
                            (h, nl, z)
                        }
                        Some(Ordering::Equal) => {
                            let nz = if *p <= z { *p } else { z };
                            (h, l, nz)
                        }
                    }
                });
            println!("{:?}", (h, l, z));
            println!("c = {:?}\n", c);
            if z < 0f64 || h < l {
                None
            }
            else if c[0] >= 0f64 { Some(vec![h]) }
            else { Some(vec![l]) }
        }
        d if mat.is_empty() => {
            Some((0..d).map(|i| if c[i] >= 0f64 { bounds[i].1 } else { bounds[i].0 }).collect())
        }
        d => {
            let rmi = (xrsh.next() as usize) % mat.len();
            println!("d = {} rmi = {}", d, rmi);
            let (a, ap) = mat[rmi].clone();
            let next_mat = (0..mat.len()).filter(|i| *i != rmi).map(|i| mat[i].clone()).collect::<Vec<_>>();
            let v = if let Some(v) = seidel_lp(xrsh, d, c, &next_mat, bounds) { v } else { return None };
            let value: f64 = (0..d)
                .map(|i| a[i] * v[i])
                .sum();
            println!("d = {} rmi = {} value = {:?} ap = {:?}", d, rmi, value, ap);
            println!("v = {:?}", v);
            println!("a[k] = {:?}", a);
            if  value <= ap {
                    Some(v)
                }
            else {
                let k = if let Some(k) = (0..d).rev().find(|i| a[*i] != 0f64) { k } else { return None };
                let next_bounds = (0..mat.len()).filter(|i| *i != k).map(|i| bounds[i].clone()).collect::<Vec<_>>();
                println!("k = {}", k);
                let mut bar_mat = next_mat.iter().map(|(b, bp)| {
                    let ratio = b[k] / a[k];
                    let v = (0..d)
                        .filter(|i| *i != k)
                        .map(|i| {
                            b[i] - ratio * a[i]
                        })
                        .collect::<Vec<_>>();
                    (v, bp - ratio * ap)
                }).collect::<Vec<_>>();
                let bar_c = (0..d)
                    .filter(|i| *i != k)
                    .map(|i| {
                        c[i] - (c[k] / a[k]) * a[i]
                    }).collect::<Vec<_>>();

                let f = (0..d).filter(|i| *i != k).map(|i| {
                    - (1f64 / a[k]) * a[i]
                }).collect::<Vec<_>>();
                let fp = bounds[k].1;
                let g = (0..d).filter(|i| *i != k).map(|i| {
                    - (1f64 / a[k]) * a[i]
                }).collect::<Vec<_>>();
                let gp = bounds[k].0;
                bar_mat.push((f, fp));
                bar_mat.push((g, gp));

                if let Some(mut v) = seidel_lp(xrsh, d - 1, &bar_c, &bar_mat, &next_bounds) {
                    v.insert(k, 0f64);
                    let s: f64 = (0..d)
                        .map(|i| a[i] * v[i])
                        .sum();
                    println!("a = {:?} ap = {:?}, {:?}", a, ap, s);
                    v[k] = (ap - s) / a[k];
                    Some(v)
                } else {
                    Some((0..d).map(|i| if c[i] >= 0f64 { bounds[i].0 } else { bounds[i].1 }).collect())
                }

            }
        }
    }
}

#[test]
fn lp_test() {
    let mut xrsh = Xorshift::new();
    let ans = seidel_lp(&mut xrsh, 2, &[1f64, 1f64], &[(vec![3f64, 1f64], 9f64), (vec![1f64, 3f64], 6f64)], &[(0f64, 10f64), (0f64, 10f64)]);
    println!("{:?}\n", ans);
}
