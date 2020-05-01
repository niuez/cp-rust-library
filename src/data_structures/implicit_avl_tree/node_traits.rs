use std::rc::Rc;
use std::cell::RefCell;
use crate::algebra::*;

#[derive(Clone, Copy)]
pub struct Position(pub usize);

pub type Link<N> = Rc<RefCell<N>>;

pub enum Child<N: Node> {
    Node(Link<N>),
    Leaf(Link<N::L>),
}


impl<N: Node> Child<N> {
    pub fn unwrap_node(self) -> Link<N> {
        if let Child::Node(n) = self { n }
        else { unreachable!("it is not node"); }
    }
    pub fn unwrap_ref_node(&self) -> &Link<N> {
        if let Child::Node(ref n) = self { n }
        else { unreachable!("it is not node"); }
    }
    pub fn parent(&self) -> Parent<N> {
        match self {
            Child::Node(ref n) => n.borrow_mut().parent().clone(),
            Child::Leaf(ref l) => l.borrow_mut().parent().clone(),
        }
    }
    pub fn parent_dir(&self) -> Option<usize> {
        match self {
            Child::Node(ref n) => {
                match n.borrow().parent() {
                    Some(p) => match *p.borrow().child(0) {
                        Child::Node(ref m) if Rc::ptr_eq(n, m) => Some(0),
                        _ => Some(1),
                    }
                    _ => None,
                }
            }
            Child::Leaf(ref l) => {
                match l.borrow().parent() {
                    Some(p) => match *p.borrow().child(0) {
                        Child::Leaf(ref m) if Rc::ptr_eq(l, m) => Some(0),
                        _ => Some(1),
                    }
                    _ => None,
                }
            }
        }
    }
    pub fn replace_parent(&self, node: Parent<N>) -> Parent<N> {
        match self {
            Child::Node(ref n) => n.borrow_mut().replace_parent(node),
            Child::Leaf(ref l) => l.borrow_mut().replace_parent(node),
        }
    }

    pub fn debug(&self, s: &str) where N: HeightNode {
        match self {
            Child::Node(ref n) => {
                println!("{}Node {:?} = {}", s, n.as_ptr(), n.borrow().height());
                println!("{}p  = {}", s, match n.borrow().parent() {
                    Some(ref p) => format!("{:?}", p.as_ptr()),
                    None => format!("None"),
                });
                n.borrow().child(0).debug(&(s.to_string() + "  "));
                n.borrow().child(1).debug(&(s.to_string() + "  "));
            }
            Child::Leaf(ref l) => {
                println!("{}Leaf {:?}", s, l.as_ptr());
                println!("{}p  = {}", s, match l.borrow().parent() {
                    Some(ref p) => format!("{:?}", p.as_ptr()),
                    None => format!("None"),
                });
            }
        }
    }
}
impl<N: Node> Clone for Child<N> {
    fn clone(&self) -> Self {
        match self {
            Child::Node(ref n) => Child::Node(n.clone()),
            Child::Leaf(ref l) => Child::Leaf(l.clone()),
        }
    }
}

impl<N: Node> PartialEq for Child<N> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Child::Node(ref nl), Child::Node(ref nr)) => Rc::ptr_eq(nl, nr),
            (Child::Leaf(ref nl), Child::Leaf(ref nr)) => Rc::ptr_eq(nl, nr),
            _ => false,
        }
    }
}

pub type Parent<N> = Option<Link<N>>;

pub trait Vertex: Sized {
    type N: Node;
    fn parent(&self) -> &Parent<Self::N>;
    fn replace_parent(&mut self, node: Parent<Self::N>) -> Parent<Self::N>;
}

pub trait Node: Vertex<N=Self> {
    type L: Vertex<N=Self> + Leaf;
    fn new(l: Child<Self>, r: Child<Self>) -> Self;
    fn push(&mut self);
    fn fix(&mut self);
    fn child(&self, dir: usize) -> &Child<Self>;
    fn child_mut(&mut self, dir: usize) -> &mut Child<Self>;
    fn replace(&mut self, dir: usize, node: Child<Self>) -> Child<Self>;
}

pub trait Leaf: Vertex where <Self as Vertex>::N: Node<L=Self> {
    type Value;
    fn value(&self) -> &Self::Value;
    fn value_mut(&mut self) -> &mut Self::Value;
}

pub trait KeySearch<K> {
    fn key_search(&self, key: K) -> Option<(usize, K)>;
}

pub trait ReversibleNode { fn reverse(&mut self); }

pub trait SizeNode { fn size(&self) -> usize; }

pub trait HeightNode { fn height(&self) -> isize; }

pub trait FoldNode where Self: Vertex, <<Self::N as Node>::L as Leaf>::Value: Monoid {
    fn fold(&self) -> <<Self::N as Node>::L as Leaf>::Value;
}

impl<N: Node + SizeNode> SizeNode for Child<N> {
    fn size(&self) -> usize {
        match *self {
            Child::Leaf(_) => 1,
            Child::Node(ref n) => n.borrow().size(),
        }
    }
}
impl<N: Node + HeightNode> HeightNode for Child<N> {
    fn height(&self) -> isize {
        match *self {
            Child::Leaf(_) => 1,
            Child::Node(ref n) => n.borrow().height(),
        }
    }
}

impl<N: Node + FoldNode> Child<N> where <<N as Node>::L as Leaf>::Value: Monoid {
    pub fn fold_child(&self) -> <<N as Node>::L as Leaf>::Value {
        match *self {
            Child::Leaf(ref l) => l.borrow().value().clone(),
            Child::Node(ref n) => n.borrow().fold(),
        }
    }
}

impl<N> KeySearch<Position> for N where N: Node + SizeNode {
    fn key_search(&self, key: Position) -> Option<(usize, Position)> {
        match self.child(0).size().cmp(&key.0) {
            std::cmp::Ordering::Greater => Some((0, key)),
            _ => Some((1, Position(key.0 - self.child(0).size()))),
        }
    }
}
