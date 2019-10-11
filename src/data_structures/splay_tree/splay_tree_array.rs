use crate::data_structures::node_traits::*;
use crate::algebra::*;

use std::cmp::Ordering::{ Equal, Greater, Less };

pub trait SplayArrayNode: Node + SizeNode {}
impl<N: Node + SizeNode> SplayArrayNode for N {}

fn depth_fix<N: SplayArrayNode>(n: &mut Link<N>, dir: usize) {
    if let Some(ref mut x) = n {
        depth_fix(x.as_mut().child_mut(dir), dir);
        x.as_mut().fix();
    }
}

fn splay<N: SplayArrayNode>(mut root: Box<N>, mut i: usize) -> Box<N> {
    let mut top_left: Link<N> = None;
    let mut top_right: Link<N> = None;
    {
        let mut left = &mut top_left;
        let mut right = &mut top_right;
        loop {
            let ([le, ri], rt) = {
                let mut x = root;
                x.as_mut().push();

                let alpha = match i.cmp(&size(x.child(0))) {
                    Equal => { root = x; break }
                    Less => { 0 }
                    Greater => { i = i - size(x.child(0)) - 1; 1 }
                };

                let mut y = x.as_mut().take(alpha).unwrap();
                y.as_mut().push();

                match i.cmp(&size(y.child(0))) {
                    Equal => {
                        if alpha == 0 { ([None, Some(x)], y) }
                        else { ([Some(x), None], y) }
                    }
                    cm => {
                        let beta = if cm == Less { 0 } else { i = i - size(y.child(0)) - 1; 1 };
                        let z = y.as_mut().take(beta).unwrap();

                        let mut res = [None, None];
                        if alpha == beta {
                            x.as_mut().set(alpha, y.as_mut().take(alpha ^ 1));
                            y.as_mut().set(alpha ^ 1, Some(x));
                            res[alpha ^ 1] = Some(y);
                        }
                        else {
                            res[alpha ^ 1] = Some(x);
                            res[beta ^ 1] = Some(y);
                        }

                        (res, z)
                    }
                }
            };
            root = rt;
            if let Some(l) = le {
                *left = Some(l);
                let t = left;
                left = t.as_mut().unwrap().as_mut().child_mut(1);
            }
            if let Some(r) = ri {
                *right = Some(r);
                let t = right;
                right = t.as_mut().unwrap().as_mut().child_mut(0);
            }
        }
        std::mem::swap(&mut root.as_mut().take(0), left);
        std::mem::swap(&mut root.as_mut().take(1), right);
    }
    depth_fix(&mut top_left, 1);
    depth_fix(&mut top_right, 0);
    root.as_mut().set(0, top_left);
    root.as_mut().set(1, top_right);
    root
}

fn merge<N: SplayArrayNode>(x: Link<N>, y: Link<N>) -> Link<N> {
    match x {
        Some(x) => {
            let sz = x.size();
            let mut x = splay(x, sz - 1);
            x.as_mut().set(1, y);
            Some(x)
        }
        None => y
    }
}

fn split<N: SplayArrayNode>(x: Link<N>, i: usize) -> (Link<N>, Link<N>) {
    assert!(i <= size(&x), "not validate spliting");
    if i == 0 { (None, x) }
    else if i == size(&x) { (x, None) }
    else {
        let mut x = splay(x.unwrap(), i);
        let y = x.as_mut().take(0);
        (y, Some(x))
    }
}

pub struct SplayArray<N: SplayArrayNode> {
    root: Link<N>,
}

impl<N: SplayArrayNode> SplayArray<N> {
    pub fn empty() -> Self { SplayArray { root: None } }
    pub fn new(node: N) -> Self { SplayArray { root: Some(Box::new(node)) } }
    pub fn len(&self) -> usize { size(&self.root) }
    pub fn merge(self, other: Self) -> Self { SplayArray { root: merge(self.root, other.root) } }
    pub fn split(self, i: usize) -> (Self, Self) {
        let (l, r) = split(self.root, i);
        ( SplayArray { root: l }, SplayArray { root: r })
    }
    pub fn at(&mut self, i: usize) -> &N::Value {
        self.root = Some(splay(self.root.take().unwrap(), i));
        self.root.as_ref().unwrap().value()
    }
    pub fn take(&mut self) -> Self {
        SplayArray { root: self.root.take() }
    }
    pub fn range<'a>(&'a mut self, ran: std::ops::Range<usize>) -> SplayRange<'a, N> {
        let (l, r) = self.take().split(ran.end);
        let (l, c) = l.split(ran.start);
        SplayRange {
            before: self,
            left: l,
            center: c,
            right: r,
        }
    }
}

impl<N: SplayArrayNode + FoldNode> SplayArray<N> where N::Value: Monoid {
    pub fn fold(&self) -> N::Value { fold(&self.root) }
}

impl<N: SplayArrayNode + ReversibleNode> SplayArray<N> {
    pub fn reverse(&mut self) { self.root.as_mut().map(|r| r.as_mut().reverse()); }
}

pub struct SplayRange<'a, N: SplayArrayNode> {
    before: &'a mut SplayArray<N>,
    left: SplayArray<N>,
    center: SplayArray<N>,
    right: SplayArray<N>,
}

impl<'a, N: SplayArrayNode> Drop for SplayRange<'a, N> {
    fn drop(&mut self) {
        self.before.root = merge(self.left.root.take(), self.right.root.take());
    }
}

impl<'a, N: SplayArrayNode> SplayRange<'a, N> {
    pub fn take(&mut self) -> SplayArray<N> { SplayArray { root: self.center.root.take() } }
    pub fn len(&self) -> usize { size(&self.center.root) }
    pub fn at(&mut self, i: usize) -> &N::Value {
        self.center.root = Some(splay(self.center.root.take().unwrap(), i));
        self.center.root.as_ref().unwrap().value()
    }
}

impl<'a, N: SplayArrayNode + FoldNode> SplayRange<'a, N> where N::Value: Monoid {
    pub fn fold(&self) -> N::Value { fold(&self.center.root) }
}

impl<'a, N: SplayArrayNode + ReversibleNode> SplayRange<'a, N> {
    pub fn reverse(&mut self) { self.center.root.as_mut().map(|r| r.as_mut().reverse()); }
}


#[cfg(test)]
mod splay_array_test {
    use crate::data_structures::node_traits::*;
    use super::*;

    struct U(usize);
    def_node! { NodeU, U; size, rev, }

    fn generate_10sp() -> SplayArray<NodeU> {
        let mut sp = SplayArray::empty();
        for i in 0..10 {
            sp = sp.merge(
                SplayArray::new( NodeU::new(U(i)) )
                );
        }
        sp
    }
    
    #[test]
    fn splay_array_test() {
        let sp = generate_10sp();
        assert_eq!(sp.len(), 10);
        let (mut l, r) = sp.split(3);
        assert_eq!(l.len(), 3);
        assert_eq!(r.len(), 7);
        l.reverse();
        let mut sp = r.merge(l);
        assert_eq!(sp.at(6).0, 9);
        assert_eq!(sp.at(7).0, 2);

        let mut sp = generate_10sp();
        {
            let mut ran = sp.range(1..5);
            assert_eq!(ran.at(0).0, 1);
            let center = ran.take();
            assert_eq!(center.len(), 4);
        }
        assert_eq!(sp.at(0).0, 0);
        assert_eq!(sp.len(), 6);
    }
}
