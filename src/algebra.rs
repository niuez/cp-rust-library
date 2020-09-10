pub trait Magma: Sized + Clone {
  fn op(&self, rhs: &Self) -> Self;
}

pub trait Associative: Magma {}

pub trait Unital: Magma {
  fn identity() -> Self;
}

pub trait Monoid: Magma + Associative + Unital {}

pub trait Reverse: Magma {
    fn reverse(&self) -> Self;
}

pub trait Inv: Magma {
    fn inv(&self) -> Self;
}

pub trait Effect<E: Monoid> {
    fn effect(&self, e: &E) -> Self;
}

impl<T: Magma + Associative + Unital> Monoid for T {}

pub trait Field:
    Sized + Copy + std::cmp::Eq +
    std::ops::Add<Output=Self> + 
    std::ops::Sub<Output=Self> +
    std::ops::Mul<Output=Self> +
    std::ops::Div<Output=Self> +
    std::ops::AddAssign<> + 
    std::ops::SubAssign<> +
    std::ops::MulAssign<> +
    std::ops::DivAssign<> +
    std::ops::Neg<Output=Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}
