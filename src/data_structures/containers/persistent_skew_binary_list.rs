use std::rc::Rc;

enum Node<T> {
    Node(Rc<SkewNode<T>>),
    Leaf(Rc<T>),
}

impl<T> Clone for Node<T> {
    fn clone(&self) -> Self {
        match *self {
            Node::Node(ref n) => Node::Node(n.clone()),
            Node::Leaf(ref n) => Node::Leaf(n.clone()),
        }
    }
}

impl<T> Node<T> {
    fn skew(&self) -> usize {
        match *self {
            Node::Node(ref n) => n.as_ref().k,
            _ => 1,
        }
    }
}

struct SkewNode<T> {
    val: T,
    k: usize,
    ch: [Node<T>; 2],
}

pub struct SkewList<T> {
    ch: Node<T>,
    next: Option<Rc<SkewList<T>>>,
}

impl<T> SkewList<T> {
    fn skew(&self) -> usize { self.ch.skew() }
}

pub struct SkewBinaryList<T> {
    root: Option<Rc<SkewList<T>>>,
}

impl<T> Clone for SkewBinaryList<T> {
    fn clone(&self) -> Self {
        Self { root: self.root.clone() }
    }
}

impl<T> SkewBinaryList<T> {
    pub fn new() -> Self {
        Self { root: None, }
    }
    pub fn cons(&self, val: T) -> Self {
        match self.root {
            None => Self {
                root: Some(Rc::new(SkewList {
                    ch: Node::Leaf(Rc::new(val)),
                    next: None,
                })),
            },
            Some(ref root) => {
                let k = root.skew();
                match root.next {
                    Some(ref nxt) if k == nxt.as_ref().skew() => Self {
                        root: Some(Rc::new(SkewList {
                            ch: Node::Node(Rc::new(SkewNode {
                                val,
                                k: k + 1,
                                ch: [root.as_ref().ch.clone(), nxt.as_ref().ch.clone()],
                            })),
                            next: nxt.as_ref().next.clone(),
                        }))
                    },
                    _ => Self {
                        root: Some(Rc::new(SkewList {
                            ch: Node::Leaf(Rc::new(val)),
                            next: Some(root.clone()),
                        }))
                    }
                }
            }
        }
    }
    pub fn at(&self, mut i: usize) -> &T {
        let mut x = self.root.as_ref().unwrap();
        while (1 << x.skew()) - 1 <= i {
            i -= (1 << x.skew()) - 1;
            x = x.as_ref().next.as_ref().unwrap();
        }
        let mut x = &x.as_ref().ch;
        loop {
            let k = x.skew();
            match *x {
                Node::Leaf(ref l) => return l.as_ref(),
                Node::Node(ref n) if i == 0 => return &n.as_ref().val,
                Node::Node(ref n) if i - 1 < (1 << (k - 1)) - 1 => {
                    i -= 1;
                    x = &n.as_ref().ch[0];
                }
                Node::Node(ref n) => {
                    i -= 1 << (k - 1);
                    x = &n.as_ref().ch[1];
                }
            }
        }
    }
}
