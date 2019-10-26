use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign };

/// Static mod for ModInt
///
/// `Mod` has a static modulo number.
pub trait Mod: Sized {
    /// Create new ModInt from integer a.
    fn new(a: u64) -> ModInt<Self> { ModInt::new(a) }
    /// The static modulo number.
    fn m() -> u64;
}

/// Define new modulo easily
///
/// This macro help define new modulo.
///
/// # Example
///
/// ```
/// use cp_rust_library::*;
/// use cp_rust_library::math::modint::*;
/// const_mod! { MOD, 17 }
/// fn main() {
///     assert_eq!(MOD::m(), 17)
/// }
/// ```
#[macro_export]
macro_rules! const_mod {
    ($st: ident, $m: expr) => {
        struct $st {}
        impl Mod for $st { fn m() -> u64 { $m } }
    }
}

/// For calculating on the modulo
pub struct ModInt<M: Mod> { a: u64, _p: std::marker::PhantomData<M> }

impl<M: Mod> ModInt<M> {
    /// Create new ModInt from `a: u64`
    pub fn new(a: u64) -> Self { ModInt { a: a % M::m() as u64, _p: std::marker::PhantomData } }
    /// Create new ModInt from `a: i64`
    pub fn newi(a: i64) -> Self { ModInt { a: (a + M::m() as i64) as u64 % M::m(), _p: std::marker::PhantomData } }
    /// Get value of ModInt as `u64`
    pub fn value(&self) -> u64 { self.a }
    /// Raises self to the power of `p` by squaring
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
    /// Get the inverse elements of self on modulo
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


#[cfg(test)]
mod modint_test {
    use super::*;

    const_mod!{ MOD, 1_000_000_007 }
    type Fp = ModInt<MOD>;

    macro_rules! fp {
        ($x:expr) => { Fp::new($x) }
    }
    
    #[test]
    fn modint_test() {
        let mut a = fp!(1) * fp!(2) * fp!(3) * fp!(4);
        assert_eq!(a.value(), 1 * 2 * 3 * 4);
        a /= fp!(2);
        assert_eq!(a.value(), 1 * 3 * 4);
    }
}
