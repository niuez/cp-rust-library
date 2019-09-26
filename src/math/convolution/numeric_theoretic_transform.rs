use crate::math::modint::{ ModInt, Mod };

use std::mem::swap;

pub trait NttMod: Mod {
    fn primitive() -> ModInt<Self>;
    fn nlimit() -> usize;
}

#[macro_export]
macro_rules! define_nttmod {
    ($st: ident, $m: expr, $pr: expr, $nl: expr) => {
        pub struct $st {}
        impl Mod for $st { fn m() -> u64 { $m } }
        impl NttMod for $st {
            fn primitive() -> ModInt<Self> { ModInt::new($pr) }
            fn nlimit() -> usize { $nl }
        }
    };
}

define_nttmod! { NttMod1224736769, 1224736769, 3, (1 << 24) }
define_nttmod! { NttMod1053818881, 1053818881, 7, (1 << 20) }
define_nttmod! { NttMod1051721729, 1051721729, 6, (1 << 20) }
define_nttmod! { NttMod1045430273, 1045430273, 3, (1 << 20) }
define_nttmod! { NttMod1012924417, 1012924417, 5, (1 << 21) }
define_nttmod! { NttMod1007681537, 1007681537, 3, (1 << 20) }
define_nttmod! { NttMod1004535809, 1004535809, 3, (1 << 21) }
define_nttmod! { NttMod998244353, 998244353, 3, (1 << 23) }
define_nttmod! { NttMod985661441, 985661441, 3, (1 << 22) }
define_nttmod! { NttMod976224257, 976224257, 3, (1 << 20) }
define_nttmod! { NttMod975175681, 975175681, 17, (1 << 21) }
define_nttmod! { NttMod962592769, 962592769, 7, (1 << 21) }
define_nttmod! { NttMod950009857, 950009857, 7, (1 << 21) }
define_nttmod! { NttMod943718401, 943718401, 7, (1 << 22) }
define_nttmod! { NttMod935329793, 935329793, 3, (1 << 22) }
define_nttmod! { NttMod924844033, 924844033, 5, (1 << 21) }
define_nttmod! { NttMod469762049, 469762049, 3, (1 << 26) }
define_nttmod! { NttMod167772161, 167772161, 3, (1 << 25) }

pub fn numeric_theoretic_transform<NM: NttMod>(arr: &[ModInt<NM>], inv: bool) -> Vec<ModInt<NM>> {
    let n = arr.len();
    assert!(n <= NM::nlimit(), "over length limit");
    assert!(n.count_ones() == 1, "the length of array is no square");
    let mut a: Vec<_> = arr.to_vec();
    let mut tmp: Vec<_> = (0..n).map(|_| ModInt::new(0)).collect();
    let mut ai: Vec<_> = (0..n).map(|i| i).collect();
    let mut ti: Vec<_> = (0..n).map(|_| 0).collect();
    let bit = n.trailing_zeros();

    for si in (0..bit).rev() {
        let s = (1 << si) as usize;
        swap(&mut a, &mut tmp);
        swap(&mut ai, &mut ti);
        let zeta = if !inv { NM::primitive().pow((NM::m() - 1) / (s << 1) as u64) }
                      else { NM::primitive().pow((NM::m() - 1) / (s << 1) as u64).pow(NM::m() - 2) };
        let mut z_i = ModInt::new(1);
        let mut ev = 0;
        let mut od = 1;
        for i in 0..n {
            if (i & s) != 0 {
                a[i] = (tmp[i - s] - tmp[i]) * z_i;
                ai[i] = ti[od];
                od += 2;
                z_i *= zeta;
            }
            else {
                a[i] = tmp[i] + tmp[i + s];
                ai[i] = ti[ev];
                ev += 2;
                z_i = ModInt::new(1);
            }
        }
    }
    swap(&mut a, &mut tmp);
    let inv_n = if !inv { ModInt::new(1) } else { ModInt::new(1) / ModInt::new(n as u64) };
    for i in 0..n { a[ai[i]] =  tmp[i] * inv_n; }
    a
}
