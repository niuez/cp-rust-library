use crate::algebra::*;

use std::ops::Range;

pub enum Node<T: Monoid> {
    Section(Box<Section<T>>),
    Leaf(Leaf<T>),
    None,
}

pub struct Section<T: Monoid> {
    left: Node<T>,
    right: Node<T>,
    val: T,
}

pub struct Leaf<T: Monoid> {
    i: usize,
    val: T,
}

impl<T: Monoid> Leaf<T> {
    fn new(i: usize, x: T) -> Self {
        Leaf { i: i, val: x }
    }
    fn fold(&self) -> T { self.val.clone() }
}

impl<T: Monoid> Section<T> {
    fn new() -> Self {
        Section {
            left: Node::None,
            right: Node::None,
            val: T::identity(),
        }
    }
    fn fold(&self) -> T { self.val.clone() }
    fn update(&mut self, i: usize, x: T, l: usize, r: usize) {
        let m = (l + r) >> 1;
        if i < m {
            let left = self.left.take();
            self.left = left.update(i, x, l, m);
        }
        else {
            let right = self.right.take();
            self.right = right.update(i, x, m, r);
        }
        self.val = self.left.fold().op(&self.right.fold());
    }
}

impl<T: Monoid> Node<T> {
    fn take(&mut self) -> Node<T> {
        std::mem::replace(self, Node::None)
    }
    fn fold(&self) -> T {
        match self {
            &Node::Section(ref sec) => sec.as_ref().fold(),
            &Node::Leaf(ref leaf) => leaf.fold(),
            &Node::None => T::identity(),
        }
    }
    fn update(self, i: usize, x: T, l: usize, r: usize) -> Self {
        match self {
            Node::Section(mut sec) => {
                sec.as_mut().update(i, x, l, r);
                Node::Section(sec)
            }
            Node::Leaf(leaf) => {
                if leaf.i == i {
                    Node::Leaf(Leaf::new(i, x))
                }
                else {
                    let mut new_section = Section::new();
                    let m = (l + r) >> 1;
                    if leaf.i < m {
                        new_section.left = Node::Leaf(leaf);
                    }
                    else {
                        new_section.right = Node::Leaf(leaf);
                    }
                    new_section.update(i, x, l, r);
                    Node::Section(Box::new(new_section))
                }
            }
            Node::None => {
                Node::Leaf(Leaf::new(i, x))
            }
        }
    }
    fn range_fold(&self, a: usize, b: usize, l: usize, r: usize) -> T {
        match self {
            &Node::Section(ref sec) => {
                if b <= l || r <= a { T::identity() }
                else if a <= l && r <= b { sec.fold() }
                else {
                    let m = (l + r) >> 1;
                    sec.left.range_fold(a, b, l, m).op(&sec.right.range_fold(a, b, m, r))
                }
            }
            &Node::Leaf(ref leaf) => {
                if a <= leaf.i && leaf.i < b { leaf.fold() }
                else { T::identity() }
            }
            &Node::None => T::identity(),
        }
    }
}

pub struct DynamicSegmentTree<T: Monoid> {
    root: Node<T>,
    n: usize,
}

impl<T: Monoid> DynamicSegmentTree<T> {
    pub fn new(sz: usize) -> Self {
        let mut n = 1;
        while n < sz { n = n << 1; }
        DynamicSegmentTree {
            root: Node::None,
            n: n,
        }
    }
    pub fn set(&mut self, i: usize, x: T) {
        let r = self.root.take();
        self.root = r.update(i, x, 0,  self.n);
    }
    pub fn fold(&self, ran: Range<usize>) -> T {
        self.root.range_fold(ran.start, ran.end, 0, self.n)
    }
}
