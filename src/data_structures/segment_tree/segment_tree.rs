use crate::algebra::*;

use std::ops::Range;

pub struct SegmentTree<T: Monoid> {
    node: Vec<T>,
    sz: usize,
}

impl<T: Monoid> SegmentTree<T> {
    pub fn init(arr: &[T]) -> Self {
        let mut sz = 1;
        while sz < arr.len() { sz *= 2; }
        let mut node = vec![T::identity(); sz << 1];
        for i in 0..arr.len() { node[i + sz] = arr[i].clone(); }
        for i in (1..sz).rev() { node[i] = node[i << 1].op(&node[(i << 1) + 1]); }
        SegmentTree { node: node, sz: sz }
    }

    fn fix(&mut self, i: usize) {
        let mut idx = i;
        while idx > 1 {
            idx = idx >> 1;
            self.node[idx] = self.node[idx << 1].op(&self.node[(idx << 1)  + 1]);
        }
    }

    pub fn get(&self, i: usize) -> &T { &self.node[i] }

    pub fn set(&mut self, i: usize, x: T) {
        let idx = i + self.sz;
        self.node[idx] = x;
        self.fix(idx);
    }

    pub fn op_mut(&mut self, i: usize, x: &T) {
        let idx = i + self.sz;
        self.node[idx] = self.node[idx].op(x);
        self.fix(idx);
    }

    pub fn fold(&self, ran: Range<usize>) -> T {
        let mut lx = T::identity();
        let mut rx = T::identity();
        let mut l = ran.start + self.sz;
        let mut r = ran.end + self.sz;
        while l < r {
            if (l & 1) == 1 { lx = lx.op(&self.node[l]); }
            if (r & 1) == 1 { rx = self.node[r - 1].op(&rx); }
            l = (l + 1) >> 1;
            r = r >> 1;
        }
        lx.op(&rx)
    }
}

#[cfg(test)]
mod rsq_test {
    use crate::algebra::*;
    use crate::data_structures::segment_tree::segment_tree::SegmentTree;

    #[derive(Clone)]
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
        let mut seg = SegmentTree::init(&vec![Am(1), Am(2), Am(3)]);
        assert!(seg.fold(0..2).0 == 3);
        assert!(seg.fold(1..2).0 == 2);
        seg.set(1, Am(5));
        assert!(seg.fold(0..2).0 == 6);
        assert!(seg.fold(1..2).0 == 5);
        seg.op_mut(1, &Am(5));
        assert!(seg.fold(0..2).0 == 11);
        assert!(seg.fold(1..2).0 == 10);
    }
    #[test]
    fn corner_test() {
        let seg = SegmentTree::init(&vec![Am(1)]);
        assert!(seg.fold(0..1).0 == 1);
    }
}
