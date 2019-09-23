pub trait Magma: Sized + Clone {
  fn op(&self, rhs: &Self) -> Self;
}

pub trait MutMagma: Magma {
    fn mut_op(&mut self, rhs: &Self) { *self = self.clone().op(rhs); }
}

pub trait Associative: Magma {}

pub trait Unital: Magma {
  fn identity() -> Self;
}

pub trait Monoid: Magma + Associative + Unital {}

pub trait Effect<E: Monoid> where Self: Monoid {
    fn effect(&self, e: &E) -> Self;
}

pub trait MutEffect<E: Monoid> where Self: Effect<E> {
    fn mut_effect(&mut self, e: &E) { *self = self.clone().effect(e); }
}

impl<T: Magma + Associative + Unital> Monoid for T {}
