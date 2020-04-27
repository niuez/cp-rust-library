use crate::algebra::*;

pub type Link<N> = Option<Box<N>>;

pub trait Node: Sized {
    type Value;
    fn push(&mut self);
    fn fix(&mut self);
    fn child(&self, dir: usize) -> &Link<Self>;
    fn child_mut(&mut self, dir: usize) -> &mut Link<Self>;
    fn replace(&mut self, dir: usize, node: Link<Self>) -> Link<Self>;
    fn value(&self) -> &Self::Value;
    fn value_mut(&mut self) -> &mut Self::Value;
}

pub trait ReversibleNode { fn reverse(&mut self); }

pub trait SizeNode { fn size(&self) -> usize; }

pub trait HeightNode { fn height(&self) -> isize; }

pub trait FoldNode where Self: Node, <Self as Node>::Value: Monoid {
    fn fold(&self) -> <Self as Node>::Value;
}

impl<N> SizeNode for Link<N> where N: SizeNode {
    fn size(&self) -> usize {
        match *self {
            Some(ref node) => node.size(),
            None => 0,
        }
    }
}

impl<N> HeightNode for Link<N> where N: HeightNode {
    fn height(&self) -> isize {
        match *self {
            Some(ref node) => node.height(),
            None => 0,
        }
    }
}
