use crate::algebra::*;

pub type Link<N> = Option<Box<N>>;

pub trait Node: Sized {
    type Value;
    fn push(&mut self);
    fn fix(&mut self);
    fn child(&self, dir: usize) -> &Link<Self>;
    fn child_mut(&mut self, dir: usize) -> &mut Link<Self>;
    fn take(&mut self, dir: usize) -> Link<Self>;
    fn set(&mut self, dir: usize, node: Link<Self>);
    fn value(&self) -> &Self::Value;
    fn value_mut(&mut self) -> &mut Self::Value;
}

pub trait ReversibleNode: Node { fn reverse(&mut self); }
pub trait SizeNode: Node { fn size(&self) -> usize; }
pub trait FoldNode where Self: Node, <Self as Node>::Value: Monoid {
    fn fold(&self) -> <Self as Node>::Value;
}

pub fn size<N: SizeNode>(link: &Link<N>) -> usize {
    match *link {
        Some(ref node) => node.size(),
        None => 0,
    }
}
pub fn fold<N: FoldNode>(link: &Link<N>) -> <N as Node>::Value where <N as Node>::Value: Monoid {
    match *link {
        Some(ref node) => node.fold(),
        None => <N as Node>::Value::identity(),
    }
}
