use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign };

pub fn inv_mod(a: u64, m: u64) -> u64 {
    let m = m as i64;
    let mut a = a as i64;
    let mut b = m as i64;
    let mut u = 1i64;
    let mut v = 0i64;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        u -= t * v;
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut u, &mut v);
    }
    let ans = (if u >= 0 { u % m } else { (m + (u % m)) % m }) as u64;
    ans
}

pub trait Mod: Sized {
    fn m() -> u32;
    fn m64() -> u64;
    fn mi64() -> i64;
}

#[macro_export]
macro_rules! const_mod {
    ($st: ident, $m: expr) => {
        struct $st {}
        impl Mod for $st {
            fn m() -> u32 { $m }
            fn m64() -> u64 { $m as u64 }
            fn mi64() -> i64 { $m as i64 }
        }
    }
}

pub struct ModInt<M: Mod> { a: u32, _p: std::marker::PhantomData<M> }

impl<M: Mod> ModInt<M> {
    pub fn new(a: u32) -> Self { ModInt { a, _p: std::marker::PhantomData } }
    pub fn newu64(a: u64) -> Self { ModInt { a: (a % M::m64()) as u32, _p: std::marker::PhantomData } }
    pub fn newi64(a: i64) -> Self { ModInt { a: (((a % M::mi64()) + M::mi64()) % M::mi64()) as u32, _p: std::marker::PhantomData } }
    pub fn value(&self) -> u32 { self.a }
    pub fn pow(&self, p: u64) -> Self {
        let mut exp = p;
        let mut now = *self;
        let mut ans = ModInt::new(1);
        while exp > 0 {
            if (exp & 1) == 1 { ans *= now; }
            now *= now;
            exp >>= 1;
        }
        ans
    }
    pub fn inv(&self) -> Self { Self::new(inv_mod(self.a as u64 , M::m64()) as u32) }
}

impl<M: Mod> Clone for ModInt<M> { fn clone(&self) -> Self { ModInt::new(self.a) } }
impl<M: Mod> Copy for ModInt<M> {}
impl<M: Mod> From<i64> for ModInt<M> {
    fn from(i: i64) -> Self { Self::newi64(i) }
}

impl<M: Mod> Add for ModInt<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let a = self.a + rhs.a;
        ModInt::new(if a >= M::m() { a - M::m() } else { a })
    }
}

impl<M: Mod> Sub for ModInt<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        ModInt::new(if self.a < rhs.a { M::m() + self.a - rhs.a } else { self.a - rhs.a })
    }
}

impl<M: Mod> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        ModInt::newu64(self.a as u64 * rhs.a as u64)
    }
}

impl<M: Mod> Div for ModInt<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}

impl<M: Mod> AddAssign for ModInt<M> { fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; } }
impl<M: Mod> SubAssign for ModInt<M> { fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; } }
impl<M: Mod> MulAssign for ModInt<M> { fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; } }
impl<M: Mod> DivAssign for ModInt<M> { fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; } }

impl<M: Mod> std::fmt::Debug for ModInt<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "M{}", self.a)
    }
}


impl<M: Mod> crate::algebra::Field for ModInt<M> {
    fn zero() -> Self {
        ModInt::new(0)
    }
    fn one() -> Self {
        ModInt::new(1)
    }
}
