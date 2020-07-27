use crate::algebra::*;
use crate::data_structures::set::bitset::Bitset;

pub struct LazySegmentTree<T: Monoid + Effect<E>, E: Monoid> {
    node: Box<[T]>,
    lazy: Box<[E]>,
    flag: Bitset,
    sz: usize,
    h: usize,
}

impl<T: Monoid + Effect<E>, E: Monoid> LazySegmentTree<T, E> {
    pub fn new(arr: &[T]) -> Self {
        let mut sz = 1;
        let mut h = 1;
        while sz < arr.len() { sz *= 2; h += 1; }
        let mut node = vec![T::identity(); sz << 1];
        for i in 0..arr.len() { node[i + sz] = arr[i].clone(); }
        for i in (1..sz).rev() { node[i] = node[i << 1].op(&node[(i << 1) + 1]); }
        Self {
            node: node.into_boxed_slice(),
            lazy: vec![E::identity(); sz << 1].into_boxed_slice(),
            flag: Bitset::new(sz << 1),
            sz,
            h
        }
    }

    fn eval(&self, i: usize) -> T {
        if self.flag.get(i) {
            self.node[i].effect(&self.lazy[i])
        }
        else {
            self.node[i].clone()
        }
    }

    fn effect(&mut self, i: usize, e: &E) {
        if i < self.node.len() {
            self.flag.set(i, true);
            self.lazy[i] = self.lazy[i].op(e);
        }
    }

    fn push(&mut self, i: usize) {
        if self.flag.get(i) {
            self.node[i] = self.eval(i);
            let e = std::mem::replace(&mut self.lazy[i], E::identity());
            self.effect(i << 1, &e);
            self.effect((i << 1) + 1, &e);
            self.flag.set(i, false);
        }
    }

    fn infuse(&mut self, i: usize) {
        let mut i = i >> (i.trailing_zeros() as usize);
        while i >= 1 {
            i >>= 1;
            self.node[i] = self.eval(i << 1).op(&self.eval((i << 1) + 1));
        }
    }

    fn infiltrate(&mut self, i: usize) {
        if i < self.sz << 1 {
            let ih = i.trailing_zeros() as usize;
            for j in (ih..self.h).rev() {
                self.push(i >> j);
            }
        }
    }

    pub fn update(&mut self, a: usize, b: usize, e: E) {
        let mut l = a + self.sz;
        let mut r = b + self.sz;
        self.infiltrate(l);
        self.infiltrate(r);
        while l < r {
            if l & 1 != 0 {
                self.effect(l, &e);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                self.effect(r, &e);
            }
            l >>= 1;
            r >>= 1;
        }
        self.infuse(a + self.sz);
        self.infuse(b + self.sz);
    }

    pub fn fold(&mut self, a: usize, b: usize) -> T {
        let mut l = a + self.sz;
        let mut r = b + self.sz;
        self.infiltrate(l);
        self.infiltrate(r);
        let mut lx = T::identity();
        let mut rx = T::identity();
        while l < r {
            if l & 1 != 0 {
                lx = lx.op(&self.eval(l));
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                rx = self.eval(r).op(&rx);
            }
            l >>= 1;
            r >>= 1;
        }
        lx.op(&rx)
    }
}

#[cfg(test)]
mod rmq_ruq_test {
    use crate::algebra::*;
    use crate::data_structures::segment_tree::lazy_segment_tree::LazySegmentTree;
    use std::cmp::min;

    #[derive(Clone)]
    struct Mm(usize);

    impl Magma for Mm {
        fn op(&self, right: &Self) -> Self { Mm(min(self.0, right.0)) }
    }
    impl Associative for Mm {}
    impl Unital for Mm {
        fn identity() -> Self { Mm(std::usize::MAX) }
    }

    #[derive(Clone)]
    struct Uq(Option<usize>);

    impl Magma for Uq {
        fn op(&self, right: &Self) -> Self {
            if right.0.is_none() { self.clone() }
            else { right.clone() }
        }
    }
    impl Associative for Uq {}
    impl Unital for Uq {
        fn identity() -> Self { Uq(None) }
    }
    impl Effect<Uq> for Mm {
        fn effect(&self, u: &Uq) -> Mm {
            match *u {
                Uq(Some(x)) => Mm(x),
                _ => self.clone(),
            }
        }
    }

    #[test]
    fn rmq_ruq_test() {
        let mut seg = LazySegmentTree::new(&vec![Mm::identity(); 3]);
        seg.update(0, 2, Uq(Some(1)));
        seg.update(1, 3, Uq(Some(3)));
        seg.update(2, 3, Uq(Some(2)));
        assert_eq!(seg.fold(0, 3).0, 1);
        assert_eq!(seg.fold(1, 3).0, 2);
    }
}
