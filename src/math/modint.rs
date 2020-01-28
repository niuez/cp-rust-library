use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign };

pub trait Mod: Sized {
    fn new(a: u64) -> ModInt<Self> { ModInt::new(a) }
    fn newi(a: i64) -> ModInt<Self> { ModInt::newi(a) }
    fn m() -> u64;
}

#[macro_export]
macro_rules! const_mod {
    ($st: ident, $m: expr) => {
        struct $st {}
        impl Mod for $st { fn m() -> u64 { $m } }
    }
}

pub struct ModInt<M: Mod> { a: u64, _p: std::marker::PhantomData<M> }

impl<M: Mod> ModInt<M> {
    pub fn new(a: u64) -> Self { ModInt { a: a % M::m() as u64, _p: std::marker::PhantomData } }
    pub fn newi(a: i64) -> Self { ModInt { a: (a + M::m() as i64) as u64 % M::m(), _p: std::marker::PhantomData } }
    pub fn value(&self) -> u64 { self.a }
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
    pub fn inv(&self) -> Self { self.pow(M::m() - 2) }
}

impl<M: Mod> Clone for ModInt<M> { fn clone(&self) -> Self { ModInt::new(self.a) } }
impl<M: Mod> Copy for ModInt<M> {}
impl<M: Mod> From<i64> for ModInt<M> { fn from(a: i64) -> Self { ModInt::newi(a) } }

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
        ModInt::new((self.a * rhs.a) % M::m())
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
