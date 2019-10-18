use crate::math::modint::{ ModInt, Mod };

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

pub fn numeric_theoretic_transform<NM: NttMod>(arr: &[ModInt<NM>]) -> Vec<ModInt<NM>> {
    let n = arr.len();
    assert!(n <= NM::nlimit(), "over length limit");
    assert!(n.count_ones() == 1, "the length of array is no square");
    let mut a: Vec<_> = arr.to_vec();
    let bit = n.trailing_zeros();
 
    for si in (0..bit).rev() {
        let s = (1 << si) as usize;
        let zeta = NM::primitive().pow((NM::m() - 1) / (s << 1) as u64);
        for ii in 0..(n / (s << 1)) {
            let i = ii * (s << 1);
            let mut z_i = ModInt::new(1);
            for j in 0..s {
                let t = a[i + j] - a[s + i + j];
                a[i + j] = a[i + j] + a[s + i + j];
                a[s + i + j] = t * z_i;
                z_i *= zeta;
            }
        }
    }
    a
}
 
pub fn inverse_numeric_theoretic_transform<NM: NttMod>(arr: &[ModInt<NM>]) -> Vec<ModInt<NM>> {
    let n = arr.len();
    assert!(n <= NM::nlimit(), "over length limit");
    assert!(n.count_ones() == 1, "the length of array is no square");
    let mut a: Vec<_> = arr.to_vec();
    let bit = n.trailing_zeros();
 
    for si in 0..bit {
        let s = (1 << si) as usize;
        let zeta = NM::primitive().pow((NM::m() - 1) / (s << 1) as u64).pow(NM::m() - 2);
        for ii in 0..(n / (s << 1)) {
            let i = ii * (s << 1);
            let mut z_i = ModInt::new(1);
            for j in 0..s {
                let t = a[s + i + j] * z_i;
                a[s + i + j] = a[i + j] - t;
                a[i + j] = a[i + j] + t;
                z_i *= zeta;
            }
        }
    }
    let inv_n = ModInt::new(1) / ModInt::new(n as u64);
    a.iter().map(|&x| x * inv_n).collect()
}
