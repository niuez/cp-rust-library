pub trait Magma: Sized + Clone {
  fn op(&self, rhs: &Self) -> Self;
}

pub trait Associative: Magma {}

pub trait Unital: Magma {
  fn identity() -> Self;
}

pub trait Monoid: Magma + Associative + Unital {}

pub trait Pow: Magma {
    fn pow(&self, p: usize) -> Self;
}

pub trait Effect<E: Monoid> {
    fn effect(&self, e: &E) -> Self;
}

impl<T: Magma + Associative + Unital> Monoid for T {}
