use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign };

pub struct RtModInt { a: i64, m: i64, }

impl RtModInt {
    pub fn new(a: i64, m: i64) -> Self { RtModInt { a: a % m, m: m} }
    pub fn value(&self) -> i64 { self.a }
    pub fn pow(&self, p: i64) -> Self {
        let mut exp = p;
        let mut now = *self;
        let mut ans = RtModInt::new(1, self.m);
        while exp > 0 {
            if (exp & 1) == 1 { ans *= now; }
            now *= now;
            exp >>= 1;
        }
        ans
    }
}

impl Clone for RtModInt { fn clone(&self) -> Self { RtModInt::new(self.a, self.m) } }
impl Copy for RtModInt {}

impl Add for RtModInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        assert!(self.m == rhs.m, "mods are not same");
        let v = self.a + rhs.a;
        RtModInt::new(if v >= self.m { v - self.m } else { v }, self.m)
    }
}

impl Sub for RtModInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        assert!(self.m == rhs.m, "mods are not same");
        let v = self.a - rhs.a;
        RtModInt::new(if v < 0 { v + self.m } else { v }, self.m)
    }
}

impl Mul for RtModInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        assert!(self.m == rhs.m, "mods are not same");
        RtModInt::new((self.a * rhs.a) % self.m, self.m)
    }
}

impl Div for RtModInt {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        assert!(self.m == rhs.m, "mods are not same");
        self * rhs.pow(self.m - 2)
    }
}

impl AddAssign for RtModInt { fn add_assign(&mut self, rhs: Self) { *self = *self + rhs } }
impl SubAssign for RtModInt { fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs } }
impl MulAssign for RtModInt { fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs } }
impl DivAssign for RtModInt { fn div_assign(&mut self, rhs: Self) { *self = *self / rhs } }
