use crate::algebra::*;

pub enum Link<N> {
    Some(Box<N>),
    None,
    Dummy,
}

impl<N> Link<N> {
    pub fn unwrap(self) -> Box<N> {
        if let Link::Some(n) = self { n }
        else { unreachable!(); }
    }
    pub fn take(&mut self) -> Self {
        std::mem::replace(self, Link::None)
    }
    pub fn as_ref(&self) -> Option<&Box<N>> {
        match *self {
            Link::Some(ref n) => Some(n),
            _ => None,
        }
    }
    pub fn as_mut(&mut self) -> Option<&mut Box<N>> {
        match *self {
            Link::Some(ref mut n) => Some(n),
            _ => None,
        }
    }
}

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
            Link::Some(ref node) => node.size(),
            _ => 0,
        }
    }
}

impl<N> HeightNode for Link<N> where N: HeightNode {
    fn height(&self) -> isize {
        match *self {
            Link::Some(ref node) => node.height(),
            Link::None => 0,
            Link::Dummy => std::isize::MAX,
        }
    }
}
