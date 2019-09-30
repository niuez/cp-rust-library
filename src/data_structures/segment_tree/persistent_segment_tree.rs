use crate::algebra::*;

use std::rc::Rc;
use std::ops::Range;

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T: Monoid> {
    data: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Monoid> Node<T> {
    fn new(data: T) -> Self {
        Node { data: data, left: None, right: None }
    }
    fn build(l: usize, r: usize, arr: &[T]) -> Self {
        if l + 1 >= r { Node::new(arr[l].clone()) }
        else {
            Node {
                data: T::identity(),
                left: Some(Rc::new(Node::build(l, (l + r) >> 1, arr))),
                right: Some(Rc::new(Node::build((l + r) >> 1, r, arr))),
            }
        }
    }
    fn update(&self, i: usize, x: T, l: usize, r: usize) -> Self {
        assert!(l <= i && i < r);
        if i == l && i + 1 == r { Node::new(x) }
        else if l <= i && i < ((l + r) >> 1) {
            let left = Some(Rc::new(self.left.as_ref().unwrap().update(i, x, l, (l + r) >> 1)));
            let right = self.right.clone();
            Node {
                data: match left.as_ref() { Some(n) => n.data.clone(), None => T::identity() }
                      .op(& match right.as_ref() { Some(n) => n.data.clone(), None => T::identity() }),
                left: left,
                right: right,
            }
        }
        else {
            let left = self.left.clone();
            let right = Some(Rc::new(self.right.as_ref().unwrap().update(i, x, (l + r) >> 1, r)));
            Node {
                data: match left.as_ref() { Some(n) => n.data.clone(), None => T::identity() }
                      .op(& match right.as_ref() { Some(n) => n.data.clone(), None => T::identity() }),
                left: left,
                right: right,
            }
        }
    }
    fn fold(&self, a: usize, b: usize, l: usize, r: usize) -> T {
        if a <= l && r <= b { self.data.clone() }
        else if r <= a || b <= l { T::identity() }
        else {
            match self.left.as_ref() { Some(n) => n.fold(a, b, l, (l + r) >> 1), None => T::identity() }
                .op(& match self.right.as_ref() { Some(n) => n.fold(a, b, (l + r) >> 1, r), None => T::identity() })
        }
    }
}

impl<T: Monoid> Drop for Node<T> {
    fn drop(&mut self) {
        if let Some(left) = self.left.take() {
            if let Ok(_) = Rc::try_unwrap(left) {}
        }
        if let Some(right) = self.right.take() {
            if let Ok(_) = Rc::try_unwrap(right) {}
        }
    }
}

pub struct PersistentSegmentTree<T: Monoid> {
    root: Node<T>,
    sz: usize,
}

impl<T: Monoid> PersistentSegmentTree<T> {
    pub fn new(arr: &[T]) -> Self {
        Self { root: Node::build(0, arr.len(), arr), sz: arr.len() }
    }
    pub fn set(&self, i: usize, x: T) -> Self {
        Self { root: self.root.update(i, x, 0, self.sz), sz: self.sz }
    }
    pub fn fold(&self, ran: Range<usize>) -> T {
        self.root.fold(ran.start, ran.end, 0, self.sz)
    }
}

#[cfg(test)]
mod rsq_test {
    use crate::algebra::*;
    use crate::data_structures::segment_tree::persistent_segment_tree::*;

    #[derive(Clone, Debug)]
    struct Am(usize);

    impl Magma for Am {
        fn op(&self, right: &Self) -> Self { Am(self.0 + right.0) }
    }
    impl Associative for Am {}

    impl Unital for Am {
        fn identity() -> Self { Am(0) }
    }
    #[test]
    fn rsq_test() {
        let seg = PersistentSegmentTree::new(&[Am(1), Am(2), Am(3)]);
        assert_eq!(seg.fold(0..2).0, 3);
        assert_eq!(seg.fold(1..2).0, 2);
    }
}
