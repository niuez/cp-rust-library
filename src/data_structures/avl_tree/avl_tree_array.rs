use crate::data_structures::node_traits::*;
use crate::algebra::*;

use std::cmp::Ordering::Greater;

pub trait AVLArrayNode: Node + SizeNode + HeightNode {}
impl<N: Node + SizeNode + HeightNode> AVLArrayNode for N {}

fn rotate<N: AVLArrayNode>(mut x: Box<N>, dir: usize) -> Box<N> {
    let mut y = x.replace(dir ^ 1, None).unwrap();
    y.push();
    x.replace(dir ^ 1, y.replace(dir, None));
    x.fix();
    y.replace(dir, Some(x));
    y.fix();
    y
}

fn balance<N: AVLArrayNode>(mut x: Box<N>) -> Box<N> {
    x.fix();
    if x.child(0).height() - x.child(1).height() == 2 {
        let mut y = x.replace(0, None).unwrap();
        y.push();
        x.replace(0, Some(
              if y.child(0).height() - y.child(1).height() == -1 { rotate(y, 0) }
              else { y }
          )
        );
        rotate(x, 1)
    }
    else if x.child(0).height() - x.child(1).height() == -2 {
        let mut y = x.replace(1, None).unwrap();
        y.push();
        x.replace(1, Some(
              if y.child(0).height() - y.child(1).height() == -1 { rotate(y, 1) }
              else { y }
          )
        );
        rotate(x, 0)
    }
    else { x }
}

fn deepest_node<N: AVLArrayNode>(mut x: Box<N>, dir: usize) -> (Link<N>, Box<N>) {
    let mut par = None;
    x.push();
    while let Some(ch) = x.replace(dir, None) {
        x.replace(dir, par);
        par = Some(x);
        x = ch;
        x.push();
    }
    let ln = x.replace(dir ^ 1, None);
    x.fix();
    let dn = x;
    let mut x = ln;
    while let Some(mut p) = par {
        par = p.replace(dir, x);
        x = Some(balance(p));
    }
    (x, dn)
}

fn merge_dir<N: AVLArrayNode>(mut l: Link<N>, mut root: Box<N>, r: Link<N>, dir: usize) -> Link<N> {
    let mut par = None;
    while let Some(mut ll) = l {
        ll.push();
        if ll.height() - r.height() <= 1 { l = Some(ll); break }
        let ch = ll.replace(dir, par);
        par = Some(ll);
        l = ch;
    }
    root.replace(dir ^ 1, l);
    root.replace(dir, r);
    root.fix();
    l = Some(root);
    while let Some(mut p) = par {
        par = p.replace(dir, l);
        l = Some(balance(p));
    }
    l
}

fn merge<N: AVLArrayNode>(l: Link<N>, r: Link<N>) -> Link<N> {
    match l {
        Some(l) => {
            match r {
                Some(r) => {
                    match l.height().cmp(&r.height()) {
                        Greater => {
                            let (r, root) = deepest_node(r, 0);
                            merge_dir(Some(l), root, r, 1)
                        }
                        _ => {
                            let (l, root) = deepest_node(l, 0);
                            merge_dir(Some(r), root, l, 0)
                        }
                    }
                }
                None => { Some(l) }
            }
        }
        None => { r }
    }
}

fn split<N: AVLArrayNode>(mut x: Box<N>, mut pos: usize) -> (Link<N>, Link<N>) {
    if pos == x.size() { (Some(x), None) }
    else {
        let mut par = None;
        x.push();
        while x.child(0).size() != pos {
            let ch = if pos < x.child(0).size() {
                x.replace(0, par).unwrap()
            }
            else {
                pos -= x.child(0).size() + 1;
                x.replace(1, par).unwrap()
            };
            par = Some(x);
            x = ch;
            x.push();
        }
        let mut l = x.replace(0, None);
        let r = x.replace(1, None);
        let mut r = merge_dir(r, x, None, 0);
        while let Some(mut p) = par {
            if p.child(0).is_none() ||
               p.height() < p.child(0).height() {
                par = p.replace(0, None);
                r = merge_dir(p.replace(1, None), p, r, 0);
            }
            else {
                par = p.replace(1, None);
                l = merge_dir(p.replace(0, None), p, l, 0);
            }
        }
        (l, r)
    }
}

fn at<N: AVLArrayNode>(mut x: &mut Box<N>, mut pos: usize) -> &N::Value {
    x.push();
    while pos != x.child(0).size() {
        x = if pos < x.child(0).size() {
            x.child_mut(0).as_mut().unwrap()
        }
        else {
            pos -= x.child(0).size() + 1;
            x.child_mut(1).as_mut().unwrap()
        };
        x.push();
    }
    x.value()
}

fn set<N: AVLArrayNode>(mut x: Box<N>, mut pos: usize, val: N::Value) -> Box<N> {
    let mut par = None;
    x.push();
    while x.child(0).size() != pos {
        let ch = if pos < x.child(0).size() {
            x.replace(0, par).unwrap()
        }
        else {
            pos -= x.child(0).size() + 1;
            x.replace(1, par).unwrap()
        };
        par = Some(x);
        x = ch;
        x.push();
    }
    *x.value_mut() = val;
    while let Some(mut p) = par {
        par = if p.child(0).is_none() ||
            p.height() < p.child(0).height() {
                p.replace(0, Some(x))
        }
        else {
                p.replace(1, Some(x))
        };
        p.fix();
        x = p;
    }
    x
}

pub struct AVLTreeArray<N: AVLArrayNode> {
    root: Link<N>,
}

impl<N: AVLArrayNode> AVLTreeArray<N> {
    pub fn empty() -> Self {
        AVLTreeArray { root: None, }
    }
    pub fn new(mut node: N) -> Self {
        node.fix();
        AVLTreeArray { root: Some(Box::new(node)) }
    }
    pub fn merge(self, right: Self) -> Self {
        AVLTreeArray { root: merge(self.root, right.root) }
    }
    pub fn split(self, pos: usize) -> (Self, Self) {
        match self.root {
            Some(root) => {
                let (l, r) = split(root, pos);
                (AVLTreeArray { root: l }, AVLTreeArray { root: r })
            }
            None => (Self::empty(), Self::empty())
        }
    }
    pub fn at(&mut self, pos: usize) -> &N::Value {
        at(self.root.as_mut().unwrap(), pos)
    }
    pub fn set(&mut self, pos: usize, val: N::Value) {
        self.root = Some(set(self.root.take().unwrap(), pos, val));
    }
    pub fn size(&self) -> usize {
        self.root.size()
    }
}

impl<N: AVLArrayNode + ReversibleNode> AVLTreeArray<N> {
    pub fn reverse(&mut self) {
        if let Some(ref mut r) = self.root {
            r.reverse()
        }
    }
}

impl<N: AVLArrayNode + FoldNode> AVLTreeArray<N> where <N as Node>::Value: Monoid {
    pub fn fold(&self) -> N::Value {
        match self.root {
            Some(ref node) => node.fold(),
            None => <N as Node>::Value::identity(),
        }
    }
}


#[cfg(test)]
mod avlarray_normal_test {
    use crate::data_structures::node_traits::*;
    use crate::data_structures::avl_tree::avl_tree_array::AVLTreeArray;

    struct M(usize);
    def_node! { NodeTest, M; size, height, }
    
    #[test]
    fn node_macro_test() {
        let mut arr = AVLTreeArray::empty();
        for i in 0..10 {
            arr = arr.merge(AVLTreeArray::new(NodeTest::new(M(i))));
        }
        for i in 0..10 {
            assert_eq!(arr.at(i).0, i);
        }

        let (mut l, mut r) = arr.split(4);
        assert_eq!(l.size(), 4);
        assert_eq!(r.size(), 6);
        for i in 0..4 {
            assert_eq!(l.at(i).0, i);
        }
        for i in 0..6 {
            assert_eq!(r.at(i).0, i + 4);
        }
    }
}

#[cfg(test)]
mod avlarray_reverse_test {
    use crate::data_structures::node_traits::*;
    use crate::data_structures::avl_tree::avl_tree_array::AVLTreeArray;

    struct M(usize);
    def_node! { NodeTest, M; size, height, rev, }
    
    #[test]
    fn node_macro_test() {
        let mut arr = AVLTreeArray::empty();
        for i in 0..10 {
            arr = arr.merge(AVLTreeArray::new(NodeTest::new(M(i))));
        }
        for i in 0..10 {
            assert_eq!(arr.at(i).0, i);
        }

        arr.reverse();

        for i in 0..10 {
            assert_eq!(arr.at(i).0, 9 - i);
        }
    }
}

#[cfg(test)]
mod avlarray_fold_test {
    use crate::data_structures::node_traits::*;
    use crate::data_structures::avl_tree::avl_tree_array::AVLTreeArray;
    use crate::algebra::*;

    #[derive(Clone)]
    struct Am(usize);

    impl Magma for Am {
        fn op(&self, right: &Self) -> Self { Am(self.0 + right.0) }
    }
    impl Associative for Am {}

    impl Unital for Am {
        fn identity() -> Self { Am(0) }
    }
    def_node! { NodeTest, Am; size, height, fold, }
    
    #[test]
    fn node_macro_test() {
        let mut arr = AVLTreeArray::empty();
        for i in 0..10 {
            arr = arr.merge(AVLTreeArray::new(NodeTest::new(Am(i))));
        }
        for i in 0..10 {
            assert_eq!(arr.at(i).0, i);
        }

        assert_eq!(arr.fold().0, (0..10).sum());
        let (l, r) = arr.split(4);
        assert_eq!(l.size(), 4);
        assert_eq!(r.size(), 6);
        assert_eq!(l.fold().0, (0..4).sum());
        assert_eq!(r.fold().0, (4..10).sum());
    }
}