use std::ops::{ Add, AddAssign,
                Sub, SubAssign,
                Mul, MulAssign,
                Div, DivAssign,
};

/// Complex number
pub struct Complex {
    pub x: f64,
    pub y: f64,
}

impl Complex {
    /// Create a new complex number from `x + iy`.
    pub fn new(x: f64, y: f64) -> Self { Complex { x: x, y: y } }
    /// Create a new complex number from `r * e^(i * theta)`>
    pub fn polar(r: f64, theta: f64) -> Self { Complex::new(r * theta.cos(), r * theta.sin()) }
    /// Get the conjugate complex number of self.
    pub fn conj(&self) -> Self { Complex::new(self.x, -self.y) }
    /// Get the absolute value of self.
    pub fn abs(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
    /// Get the argument value of self by atan2.
    pub fn arg(&self) -> f64 { self.y.atan2(self.x) }
}

impl Clone for Complex { fn clone(&self) -> Self { Complex { x: self.x, y: self.y } }}
impl Copy for Complex {}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Complex::new(self.x + rhs.x, self.y + rhs.y) }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { Complex::new(self.x - rhs.x, self.y - rhs.y) }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self { Complex::new(self.x * rhs.x - self.y * rhs.y, self.x * rhs.y + self.y * rhs.x) }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let z = self * rhs.conj();
        let a = rhs.x * rhs.x + rhs.y * rhs.y;
        Complex::new(z.x / a, z.y / a)
    }
}

impl AddAssign for Complex { fn add_assign(&mut self, rhs: Self) { *self = *self + rhs } }
impl SubAssign for Complex { fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs } }
impl MulAssign for Complex { fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs } }
impl DivAssign for Complex { fn div_assign(&mut self, rhs: Self) { *self = *self / rhs } }
