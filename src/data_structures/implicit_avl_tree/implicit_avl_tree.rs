use crate::data_structures::implicit_avl_tree::node_traits::*;
use std::rc::Rc;
use std::cell::{ RefCell, Ref };

pub trait AVLNode: Node + HeightNode {}
impl<N: Node + HeightNode> AVLNode for N {}


fn rotate<N: AVLNode>(x: &Link<N>, dir: usize) {
    let p_dir = Child::Node(x.clone()).parent_dir();
    let n = x
             .borrow()
             .child(dir ^ 1)
             .unwrap_ref_node()
             .borrow_mut()
             .replace(dir, Child::Node(x.clone()));
    let y = x.borrow_mut()
             .replace(dir ^ 1, n.clone()).unwrap_node();

    n.replace_parent(Some(x.clone()));
    let p = x.borrow_mut().replace_parent(Some(y.clone()));
    y.borrow_mut().replace_parent(p.clone());

    x.borrow_mut().fix();
    y.borrow_mut().fix();
    if let (Some(p), Some(p_dir)) = (p, p_dir) {
        p.borrow_mut().replace(p_dir, Child::Node(y));
    }
}

fn balance<N: AVLNode>(x: &Link<N>) {
    x.borrow_mut().fix();
    let diff = x.borrow().child(0).height() - x.borrow().child(1).height();
    match diff {
        2 => {
            let y = x.borrow().child(0).clone().unwrap_node();
            if y.borrow().child(0).height() - y.borrow().child(1).height() == -1 {
                rotate(&y, 0);
            }
            rotate(x, 1);
        }
        -2 => {
            let y = x.borrow().child(1).clone().unwrap_node();
            if y.borrow().child(0).height() - y.borrow().child(1).height() == 1 {
                rotate(&y, 1);
            }
            rotate(x, 0);
        }
        _ => {}
    }
}

fn merge_dir<N: AVLNode>(mut l: Child<N>, root: Option<Link<N>>, r: Child<N>, dir: usize) -> Link<N> {
    while l.height() - r.height() > 1 {
        let ll = l.unwrap_node();
        ll.borrow_mut().push();
        l = (*ll.borrow()).child(dir).clone();
    }
    let x = l.replace_parent(None);
    let root = match root {
        None if dir == 1 => Rc::new(RefCell::new(N::new(l.clone(), r.clone()))),
        None => Rc::new(RefCell::new(N::new(r.clone(), l.clone()))),
        Some(root) => {
            root.borrow_mut().replace(dir ^ 1, l.clone());
            root.borrow_mut().replace(dir, r.clone());
            root
        } 
    };
    l.replace_parent(Some(root.clone()));
    r.replace_parent(Some(root.clone()));
    root.borrow_mut().fix();
    root.borrow_mut().replace_parent(x.clone());
    if let Some(x) = x { x.borrow_mut().replace(dir, Child::Node(root.clone())); }
    let mut x = root;
    loop {
        let xx = (*x.borrow()).parent().clone();
        if let Some(xx) = xx {
            balance(&xx);
            x = xx;
        }
        else {
            break;
        }
    }
    x
}

fn merge<N: AVLNode>(l: Option<Child<N>>, r: Option<Child<N>>) -> Option<Child<N>> {
    match (l, r) {
        (Some(l), Some(r)) if l.height() >= r.height() => {
            Some(Child::Node(merge_dir(l, None, r, 1)))
        }
        (Some(l), Some(r)) => {
            Some(Child::Node(merge_dir(r, None, l, 0)))
        }
        (l, None) => {
            l
        }
        (None, r) => {
            r
        }
    }
}

fn find_root<N: AVLNode>(x: Link<N::L>) -> (Child<N>, usize) {
    let mut dir = 0;
    let mut x = Child::Leaf(x);
    while let Some(pdir) = x.parent_dir() {
        dir = (dir << 1) | pdir;
        let p = x.parent().unwrap();
        x = Child::Node(p);
    }
    (x, dir)
}

fn pushdown<N: AVLNode>(mut x: Child<N>, mut dir: usize) {
    while let Child::Node(c) = x {
        c.borrow_mut().push();
        x = c.borrow().child(dir & 1).clone();
        dir = dir >> 1;
    }
}

fn fixup<N: AVLNode>(x: Link<N::L>) {
    let mut x = Child::<N>::Leaf(x);
    while let Some(p) = x.parent() {
        p.borrow_mut().fix();
        x = Child::Node(p);
    }
}


fn split<N: AVLNode>(x: Link<N::L>) -> (Option<Child<N>>, Option<Child<N>>) {
    let mut pdir = Child::<N>::Leaf(x.clone()).parent_dir();
    let mut p = (*x.borrow_mut()).replace_parent(None).clone();
    let mut l = None;
    let mut r = Some(Child::Leaf(x));
    while let (Some(dir), Some(x)) = (pdir, p) {
        pdir = Child::Node(x.clone()).parent_dir();
        p = (*x.borrow_mut()).replace_parent(None).clone();
        match dir {
            0 => {
                r = match r {

                    Some(r) => {
                        let ll = x.borrow().child(1).clone();
                        ll.replace_parent(None);
                        Some(Child::Node(merge_dir(ll, Some(x), r, 0)))
                    }
                    None => Some(x.borrow().child(1).clone()),
                };
            }
            _ => {
                l = match l {
                    Some(l) => {
                        let rr = x.borrow().child(0).clone();
                        rr.replace_parent(None);
                        Some(Child::Node(merge_dir(rr, Some(x), l, 1)))
                    }
                    None => Some(x.borrow().child(0).clone()),
                };
            }
        }
    }
    (l, r)
}

fn at<K, N: AVLNode + KeySearch<K>>(mut x: Child<N>, mut pos: K) -> Option<Link<N::L>>
where N::L: KeySearch<K> {
    let mut sch_res = None;
    while let Some((Some((dir, np)), xx)) = match x.clone() {
        Child::Node(xx) => {
            xx.borrow_mut().push();
            Some(({ xx.clone().borrow().key_search(pos) }, xx))
        }
        Child::Leaf(xx) => {
            sch_res = xx.borrow().key_search(pos);
            None
        }
    } {
        pos = np;
        x = xx.borrow().child(dir).clone();
    }
    match x {
        Child::Leaf(x) => {
            match sch_res {
                None => {
                    Some(x)
                }
                _ => {
                    None
                }
            }
        }
        _ => unreachable!("node key_search must return Some value")
    }
}

pub struct ImplicitAVLTree<N: AVLNode> {
    root: Option<Child<N>>,
}

pub struct ImplicitAVLIterator<N: AVLNode> {
    node: Link<N::L>,
}

impl<N: AVLNode> ImplicitAVLTree<N> {
    pub fn empty() -> Self {
        ImplicitAVLTree { root: None }
    }
    pub fn new(node: N::L) -> (Self, ImplicitAVLIterator<N>) {
        let n = Rc::new(RefCell::new(node));
        (ImplicitAVLTree { root: Some(Child::Leaf(n.clone())) }, ImplicitAVLIterator { node: n })
    }
    pub fn merge(self, right: Self) -> Self {
        ImplicitAVLTree { root: merge(self.root, right.root) }
    }
    pub fn split(self, iter: &Option<ImplicitAVLIterator<N>>) -> (Self, Self) {
        match iter {
            Some(ref iter) => {
                match find_root::<N>(iter.node.clone()) {
                    (r, dir) if self.root == Some(r.clone()) => {
                        pushdown(r, dir);
                        let (l, r) = split::<N>(iter.node.clone());
                        (ImplicitAVLTree { root: l }, ImplicitAVLTree { root: r })
                    }
                    _ => unreachable!("invalid iterator")
                }
            }
            None => (self, ImplicitAVLTree { root: None })
        }
    }
    pub fn at<K>(&self, pos: K) -> Option<ImplicitAVLIterator<N>>
    where N: KeySearch<K>, N::L: KeySearch<K> {
        at(self.root.clone().unwrap(), pos).map(|n| ImplicitAVLIterator::<N> { node: n })
    }

    pub fn debug(&self) where <N::L as Leaf>::Value: std::fmt::Debug {
        match self.root {
            Some(ref root) => root.debug(""),
            None => println!("None"),
        }
    }
}
impl<N: AVLNode + SizeNode> ImplicitAVLTree<N> {
    pub fn size(&self) -> usize  {
        self.root.as_ref().map(|r| r.size()).unwrap_or(0)
    }

}

pub struct ImplicitAVLValue<'a, N: AVLNode> {
    n: Ref<'a, N::L>,
}

impl<'a, N: AVLNode> std::ops::Deref for ImplicitAVLValue<'a, N> {
    type Target = <N::L as Leaf>::Value;
    fn deref(&self) -> &Self::Target {
        self.n.value()
    }
}

impl<N: AVLNode> ImplicitAVLIterator<N> {
    pub fn val(&self) -> ImplicitAVLValue<'_, N> {
        ImplicitAVLValue { n: self.node.borrow() }
    }
    pub fn set<K>(&self, val: <<N as Node>::L as Leaf>::Value) {
        let (r, dir) = find_root::<N>(self.node.clone());
        pushdown(r, dir);
        *self.node.borrow_mut().value_mut() = val;
        fixup::<N>(self.node.clone());
    }
}

#[cfg(test)]
mod implicitavl_normal_test {
    use crate::data_structures::implicit_avl_tree::node_traits::*;
    use crate::data_structures::implicit_avl_tree::implicit_avl_tree::ImplicitAVLTree;

    def_implicit_node! { NodeTest, LeafTest, usize; size, height, }
    
    #[test]
    fn node_macro_test() {
        let mut arr = ImplicitAVLTree::<NodeTest>::empty();
        for i in 0..10 {
            println!("here {}", i);
            arr = arr.merge(ImplicitAVLTree::new(LeafTest::new(i)).0);
            for j in 0..(i + 1) {
                assert_eq!(arr.at(Position(j)).unwrap().val().clone(), j);
            }
        }
        for i in 0..10 {
            assert_eq!(arr.at(Position(i)).unwrap().val().clone(), i);
        }
        let iter = arr.at(Position(4));
        let (l, r) = arr.split(&iter);
        assert_eq!(l.size(), 4);
        assert_eq!(r.size(), 6);
        for i in 0..4 {
            assert_eq!(l.at(Position(i)).unwrap().val().clone(), i);
        }
        for i in 0..6 {
            assert_eq!(r.at(Position(i)).unwrap().val().clone(), i + 4);
        }
    }
}
