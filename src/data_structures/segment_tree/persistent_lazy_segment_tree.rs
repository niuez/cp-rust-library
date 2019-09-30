use crate::algebra::*;

use std::rc::Rc;
use std::ops::Range;

type Link<T, E> = Option<Rc<Node<T, E>>>;

struct Node<T: Monoid + Effect<E>, E: Monoid + Pow> {
    data: T,
    eff: E,
    left: Link<T, E>,
    right: Link<T, E>,
}

impl<T: Monoid + Effect<E>, E: Monoid + Pow> Node<T, E> {
    fn new(data: T) -> Self {
        Node { data: data, eff: E::identity(), left: None, right: None }
    }
    fn build(l: usize, r: usize, arr: &[T]) -> Self {
        if l + 1 >= r { Node::new(arr[l].clone()) }
        else {
            let left = Some(Rc::new(Node::<T, E>::build(l, (l + r) >> 1, arr)));
            let right = Some(Rc::new(Node::<T, E>::build((l + r) >> 1, r, arr)));
            Node {
                data: match left.as_ref() { Some(n) => n.data.clone(), None => T::identity() }
                      .op(& match right.as_ref() { Some(n) => n.data.clone(), None => T::identity() }),
                eff: E::identity(),
                left: left,
                right: right,
            }
        }
    }

    fn effect_range(&self, a: usize, b: usize, new_eff: E, l: usize, r: usize, fold_eff: E) -> Self {
        if a <= l && r <= b {
            let eff = fold_eff.op(&new_eff);
            Node {
                data: self.data.effect(&eff.pow(r - l)),
                eff: self.eff.op(&eff),
                left: self.left.clone(),
                right: self.right.clone(),
            }
        }
        else if r <= a || b <= l {
            Node {
                data: self.data.effect(&fold_eff.pow(r - l)),
                eff: self.eff.op(&fold_eff),
                left: self.left.clone(),
                right: self.right.clone(),
            }
        }
        else {
            let left = Some(Rc::new(self.left.as_ref().unwrap().effect_range(a, b, new_eff.clone(), l, (l + r) >> 1, self.eff.op(&fold_eff))));
            let right = Some(Rc::new(self.right.as_ref().unwrap().effect_range(a, b, new_eff.clone(), (l + r) >> 1, r, self.eff.op(&fold_eff))));
            Node {
                data: match left.as_ref() { Some(n) => n.data.clone(), None => T::identity() }
                      .op(& match right.as_ref() { Some(n) => n.data.clone(), None => T::identity() }),
                eff: E::identity(),
                left: left,
                right: right,
            }
        }
    }

    fn fold(&self, a: usize, b: usize, l: usize, r: usize, eff: E) -> T {
        if a <= l && r <= b { self.data.effect(&eff.pow(r - l)) }
        else if r <= a || b <= l { T::identity() }
        else {
            match self.left.as_ref() { Some(n) => n.fold(a, b, l, (l + r) >> 1, self.eff.op(&eff)), None => T::identity() }
                .op(& match self.right.as_ref() { Some(n) => n.fold(a, b, (l + r) >> 1, r, self.eff.op(&eff)), None => T::identity() })
        }
    }
}

impl<T: Monoid + Effect<E>, E: Monoid + Pow> Drop for Node<T, E> {
    fn drop(&mut self) {
        if let Some(left) = self.left.take() {
            if let Ok(_) = Rc::try_unwrap(left) {}
        }
        if let Some(right) = self.right.take() {
            if let Ok(_) = Rc::try_unwrap(right) {}
        }
    }
}


pub struct PersistentLazySegmentTree<T: Monoid + Effect<E>, E: Monoid + Pow> {
    root: Node<T, E>,
    sz: usize,
}
impl<T: Monoid + Effect<E>, E: Monoid + Pow> PersistentLazySegmentTree<T, E> {
    pub fn new(arr: &[T]) -> Self {
        Self { root: Node::build(0, arr.len(), arr), sz: arr.len() }
    }
    pub fn effect(&self, ran: Range<usize>, eff: E) -> Self {
        Self { root: self.root.effect_range(ran.start, ran.end, eff, 0, self.sz, E::identity()), sz: self.sz }
    }
    pub fn fold(&self, ran: Range<usize>) -> T {
        self.root.fold(ran.start, ran.end, 0, self.sz, E::identity())
    }
}

#[cfg(test)]
mod persistent_lazy_segment_tree_test {
    use crate::algebra::*;
    use crate::data_structures::segment_tree::persistent_lazy_segment_tree::*;

    #[derive(Clone, Debug)]
    struct Sm(usize);

    impl Magma for Sm {
        fn op(&self, right: &Self) -> Self { Sm(self.0 + right.0) }
    }
    impl Associative for Sm {}
    impl Unital for Sm {
        fn identity() -> Self { Sm(0) }
    }

    #[derive(Clone, Debug)]
    struct Aq(usize);

    impl Magma for Aq {
        fn op(&self, right: &Self) -> Self {
            Aq(self.0 + right.0)
        }
    }
    impl Associative for Aq {}
    impl Unital for Aq {
        fn identity() -> Self { Aq(0) }
    }
    impl Pow for Aq {
        fn pow(&self, p: usize) -> Self { Aq(self.0 * p) }
    }
    impl Effect<Aq> for Sm {
        fn effect(&self, t: &Aq) -> Self {
            Sm(self.0 + t.0)
        }
    }

    #[test]
    fn rsq_raq_test() {
        let seg = PersistentLazySegmentTree::new(&[Sm::identity(), Sm::identity(), Sm::identity()])
            .effect(0..2, Aq(1))
            .effect(1..3, Aq(2))
            .effect(2..3, Aq(3));
        assert_eq!(seg.fold(0..2).0, 4);
        assert_eq!(seg.fold(1..3).0, 8);
    }
}
