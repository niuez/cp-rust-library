use crate::data_structures::implicit_avl_tree::node_traits::*;
use std::rc::Rc;
use std::cell::RefCell;

pub trait AVLNode: Node + HeightNode {}
impl<N: Node + HeightNode> AVLNode for N {}


fn rotate<N: AVLNode>(x: &Link<N>, dir: usize) {
    let p_dir = x.borrow().parent_dir();
    let n = x.borrow_mut()
             .child_mut(dir ^ 1)
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
    match x.borrow().child(0).height() - x.borrow().child(1).height() {
        2 => {
            let xx = x.borrow();
            let y = xx.child(0).unwrap_ref_node();
            if y.borrow().child(0).height() - y.borrow().child(1).height() == -1 {
                rotate(&y, 0);
            }
            rotate(x, 1);
        }
        -2 => {
            let xx = x.borrow();
            let y = xx.child(1).unwrap_ref_node();
            if y.borrow().child(0).height() - y.borrow().child(1).height() == 1 {
                rotate(&y, 1);
            }
            rotate(x, 0);
        }
        _ => {}
    }
}

fn merge_dir<N: AVLNode>(mut l: Child<N>, root: Link<N>, r: Child<N>, dir: usize) -> Link<N> {
    while l.height() - r.height() > 1 {
        let ll = l.unwrap_node();
        ll.borrow_mut().push();
        l = ll.borrow().child(dir).clone();
    }
    let x = l.replace_parent(None);
    root.borrow_mut().replace(dir ^ 1, l);
    root.borrow_mut().replace(dir, r);
    root.borrow_mut().fix();
    root.borrow_mut().replace_parent(x.clone());
    if let Some(x) = x { x.borrow_mut().replace(dir, Child::Node(root.clone())); }
    let mut x = root;
    while let Some(xx) = x.clone().borrow().parent().clone() {
        x = xx;
        balance(&x);
    }
    x
}

fn merge<N: AVLNode>(l: Option<Child<N>>, r: Option<Child<N>>) -> Option<Child<N>> {
    match (l, r) {
        (Some(l), Some(r)) if l.height() >= r.height() => {
            Some(Child::Node(merge_dir(l, Rc::new(RefCell::new(N::new())), r, 1)))
        }
        (Some(l), Some(r)) => {
            Some(Child::Node(merge_dir(r, Rc::new(RefCell::new(N::new())), l, 0)))
        }
        (l, None) => {
            l
        }
        (None, r) => {
            r
        }
    }
}

fn split2<K, N: AVLNode + KeySearch<K>>(mut x: Child<N>, mut pos: K) -> (Option<Child<N>>, Option<Child<N>>)
where N::L: KeySearch<K>
{
    let mut pdir = None;
    let mut sch_res = None;
    while let Some((Some((dir, np)), xx)) = match x.clone() {
        Child::Node(xx) => Some(({ xx.clone().borrow().key_search(pos) }, xx)),
        Child::Leaf(xx) => {
            sch_res = xx.borrow().key_search(pos);
            None
        }
    } {
        pos = np;
        pdir = Some(dir);
        x = xx.borrow().child(dir).clone();
    }
    let mut p = x.replace_parent(None);
    let (mut l, mut r) = match x {
        Child::Node(x) => {
            let l = x.borrow_mut().child(0).clone();
            l.replace_parent(None);
            let r = x.borrow_mut().child(1).clone();
            r.replace_parent(None);
            (Some(l), Some(r))
        }
        Child::Leaf(x) => {
            match sch_res {
                Some((1, _)) => {
                    (Some(Child::Leaf(x)), None)
                }
                _ => {
                    (None, Some(Child::Leaf(x)))
                }
            }
        }
    };

    while let (Some(dir), Some(x)) = (pdir, p) {
        pdir = x.borrow().parent_dir();
        p = x.borrow_mut().replace_parent(None);
        match dir {
            0 => {
                r = match r {

                    Some(r) => Some(Child::Node(merge_dir(x.clone().borrow().child(1).clone(), x, r, 0))),
                    None => Some(x.borrow().child(1).clone()),
                };
            }
            _ => {
                l = match l {
                    Some(l) => Some(Child::Node(merge_dir(x.clone().borrow().child(0).clone(), x, l, 1))),
                    None => Some(x.borrow().child(0).clone()),
                };
            }
        }
    }
    (l, r)
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
    let mut pdir = x.borrow().parent_dir();
    let mut p = x.borrow_mut().replace_parent(None);
    let mut l = None;
    let mut r = Some(Child::Leaf(x));
    while let (Some(dir), Some(x)) = (pdir, p) {
        pdir = x.borrow().parent_dir();
        p = x.borrow_mut().replace_parent(None);
        match dir {
            0 => {
                r = match r {

                    Some(r) => Some(Child::Node(merge_dir(x.clone().borrow().child(1).clone(), x, r, 0))),
                    None => Some(x.borrow().child(1).clone()),
                };
            }
            _ => {
                l = match l {
                    Some(l) => Some(Child::Node(merge_dir(x.clone().borrow().child(0).clone(), x, l, 1))),
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
    pub fn split(self, iter: &ImplicitAVLIterator<N>) -> (Self, Self) {
        match find_root::<N>(iter.node.clone()) {
            (r, dir) if self.root == Some(r.clone()) => {
                pushdown(r, dir);
                let (l, r) = split::<N>(iter.node.clone());
                (ImplicitAVLTree { root: l }, ImplicitAVLTree { root: r })
            }
            _ => unreachable!("invalid iterator")
        }
    }
    pub fn at<K>(&self, pos: K) -> Option<ImplicitAVLIterator<N>>
    where N: KeySearch<K>, N::L: KeySearch<K> {
        at(self.root.clone().unwrap(), pos).map(|n| ImplicitAVLIterator::<N> { node: n })
    }
}

impl<N: AVLNode> ImplicitAVLIterator<N> {
    pub fn set<K>(&self, val: <<N as Node>::L as Leaf>::Value) {
        let (r, dir) = find_root::<N>(self.node.clone());
        pushdown(r, dir);
        *self.node.borrow_mut().value_mut() = val;
        fixup::<N>(self.node.clone());
    }
}
