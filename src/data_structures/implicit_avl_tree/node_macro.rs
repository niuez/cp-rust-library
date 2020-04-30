#[macro_export]
macro_rules! build_implicit_node_struct {
    ($node:ident, $($elem:ident: $t:ty,)*) => {
        struct $node {
            parent: Parent<$node>,
            child: [Child<$node>; 2],
            $($elem: $t),*
        }

        impl Vertex for $node {
            type N = $node;
            fn parent(&self) -> &Parent<Self::N> { &self.parent }
            fn replace_parent(&mut self, node: Parent<Self::N>) -> Parent<Self::N> { std::mem::replace(&mut self.parent, node) }
        }
    }
}

#[macro_export]
macro_rules! define_implicit_node {
    ($node:ident, $val_type:ty | $($e:ident : $t:ty,)* | ) => {
        build_implicit_node_struct! { $node, $($e: $t,)* }
    };
    ($node:ident, $val_type:ty | $($e:ident : $t:ty,)* | size, $($elem:tt)*) => {
        define_implicit_node! { $node, $val_type | $($e: $t,)* size: usize, | $($elem)* }
    };
    ($node:ident, $val_type:ty | $($e:ident : $t:ty,)* | height, $($elem:tt)*) => {
        define_implicit_node! { $node, $val_type | $($e: $t,)* height: isize, | $($elem)* }
    };
    ($node:ident, $val_type:ty | $($e:ident : $t:ty,)* | fold, $($elem:tt)*) => {
        define_implicit_node! { $node, $val_type | $($e: $t,)* fold: $val_type, | $($elem)* }
    };
    ($node:ident, $val_type:ty | $($e:ident : $t:ty,)* | rev, $($elem:tt)*) => {
        define_implicit_node! { $node, $val_type | $($e: $t,)* rev: bool, | $($elem)* }
    };
}

#[macro_export]
macro_rules! build_implicit_leaf_struct {
    ($node:ident, $leaf:ident, $val_type:ty) => {
        struct $leaf {
            parent: Parent<$node>,
            val: $val_type,
        }
        impl $leaf {
            pub fn new(val: $val_type) -> Self {
                $leaf {
                    parent: None,
                    val: val,
                }
            }
        }
        impl Vertex for $leaf {
            type N = $node;
            fn parent(&self) -> &Parent<Self::N> { &self.parent }
            fn replace_parent(&mut self, node: Parent<Self::N>) -> Parent<Self::N> { std::mem::replace(&mut self.parent, node) }
        }
    }
}

#[macro_export]
macro_rules! impl_implicit_new {
    ($node:ident, $val_type:ty, $l:ident, $r:ident | $($e:ident : $v:expr,)* | ) => {
        $node {
            parent: None,
            child: [$l, $r],
            $($e: $v),*
        }
    };
    ($node:ident, $val_type:ty, $l:ident, $r:ident | $($e:ident : $v:expr,)* | size, $($elem:tt)*) => {
        impl_implicit_new! { $node, $val_type, $l, $r | $($e: $v,)* size: 1, | $($elem)* }
    };
    ($node:ident, $val_type:ty, $l:ident, $r:ident | $($e:ident : $v:expr,)* | height, $($elem:tt)*) => {
        impl_implicit_new! { $node, $val_type, $l, $r | $($e: $v,)* height: 1, | $($elem)* }
    };
    ($node:ident, $val_type:ty, $l:ident, $r:ident | $($e:ident : $v:expr,)* | fold, $($elem:tt)*) => {
        impl_implicit_new! { $node, $val_type, $l, $r | $($e: $v,)* fold: <$val_type as Unital>::identity(), | $($elem)* }
    };
    ($node:ident, $val_type:ty, $l:ident, $r:ident | $($e:ident : $v:expr,)* | rev, $($elem:tt)*) => {
        impl_implicit_new! { $node, $val_type, $l, $r | $($e: $v,)* rev: false, | $($elem)* }
    };
}

#[macro_export]
macro_rules! impl_implicit_node_trait {
    ($node:ident, $leaf:ident, $val_type:ty, $($elem:tt)* ) => {
        impl Node for $node {
            type L = $leaf;
            fn new(l: Child<Self>, r: Child<Self>) -> Self {
                let mut n = impl_implicit_new! { $node, $val_type, l, r | | $($elem)* };
                n.fix();
                n
            }
            fn push(&mut self) {
                impl_implicit_push! { self, $val_type, $($elem)* }
            }
            fn fix(&mut self) {
                impl_implicit_fix! { self, $val_type, $($elem)* }
            }
            fn child(&self, dir: usize) -> &Child<Self> { &self.child[dir] } 
            fn child_mut(&mut self, dir: usize) -> &mut Child<Self> { &mut self.child[dir] } 
            fn replace(&mut self, dir: usize, node: Child<Self>) -> Child<Self> {
                let nn = std::mem::replace(&mut self.child[dir], node);
                nn
            }
        }
    } 
}

#[macro_export]
macro_rules! impl_implicit_leaf_trait {
    ($node:ident, $leaf:ident, $val_type:ty, $($elem:tt)* ) => {
        impl Leaf for $leaf {
            type Value = $val_type;
            fn value(&self) -> &Self::Value { &self.val }
            fn value_mut(&mut self) -> &mut Self::Value { &mut self.val }
        }
    } 
}

#[macro_export]
macro_rules! impl_implicit_push {
    ($mself:expr, $val_type:ty, ) => {};
    ($mself:expr, $val_type:ty, rev, $($elem:tt)*) => {
        if $mself.rev {
            $mself.child.swap(0, 1);
            if let Child::Node(ref ch) = *$mself.child(0) {
                ch.borrow_mut().reverse();
            }
            if let Child::Node(ref ch) = *$mself.child(1) {
                ch.borrow_mut().reverse();
            }
        }
        $mself.rev = false;
        impl_implicit_push! { $mself, $val_type, $($elem)* }
    };
    ($mself:expr, $val_type:ty, $head:tt, $($elem:tt)*) => {
        impl_implicit_push! { $mself, $val_type, $($elem)* }
    };
}

#[macro_export]
macro_rules! impl_implicit_fix {
    ($mself:expr, $val_type:ty, ) => {};
    ($mself:expr, $val_type:ty, size, $($elem:tt)*) => {
        $mself.size = $mself.child[0].size() + $mself.child[1].size();
        impl_implicit_fix! { $mself, $val_type, $($elem)* }
    };
    ($mself:expr, $val_type:ty, height, $($elem:tt)*) => {
        $mself.height = std::cmp::max($mself.child[0].height(), $mself.child[1].height()) + 1;
        impl_implicit_fix! { $mself, $val_type, $($elem)* }
    };
    ($mself:expr, $val_type:ty, fold, $($elem:tt)*) => {
        $mself.fold = $mself.child(0).fold_child().op(&$mself.child(1).fold_child());
        impl_implicit_fix! { $mself, $val_type, $($elem)* }
    };
    ($mself:expr, $val_type:ty, $head:tt, $($elem:tt)*) => {
        impl_implicit_fix! { $mself, $val_type, $($elem)* }
    };
}

#[macro_export]
macro_rules! impl_implicit_size_trait {
    ($node:ident, $leaf:ident, $val_type:ty, ) => {};
    ($node:ident, $leaf:ident, $val_type:ty, size, $($tail:tt)*) => {
        impl SizeNode for $node { fn size(&self) -> usize { self.size } }

        impl KeySearch<Position> for $leaf {
            fn key_search(&self, key: Position) -> Option<(usize, Position)> {
                match key {
                    Position(0) => None,
                    _ => Some((1, key)),
                }
            }
        }
    };
    ($node:ident, $leaf:ident, $val_type:ty, $head:tt, $($elem:tt)*) => {
        impl_implicit_size_trait! { $node, $leaf, $val_type, $($elem)* }
    };
}


#[macro_export]
macro_rules! impl_implicit_height_trait {
    ($node:ident, $val_type:ty, ) => {};
    ($node:ident, $val_type:ty, height, $($tail:tt)*) => {
        impl HeightNode for $node { fn height(&self) -> isize { self.height } }
    };
    ($node:ident, $val_type:ty, $head:tt, $($elem:tt)*) => {
        impl_implicit_height_trait! { $node, $val_type, $($elem)* }
    };
}

#[macro_export]
macro_rules! impl_implicit_fold_trait {
    ($node:ident, $val_type:ty, ) => {};
    ($node:ident, $val_type:ty, fold, $($tail:tt)*) => {
        impl FoldNode for $node { fn fold(&self) -> $val_type { self.fold.clone() } }
    };
    ($node:ident, $val_type:ty, $head:tt, $($elem:tt)*) => {
        impl_implicit_fold_trait! { $node, $val_type, $($elem)* }
    };
}

#[macro_export]
macro_rules! def_implicit_node {
    ($node:ident, $leaf:ident, $val_type:ty; $($elem:tt)* ) => {
        define_implicit_node! { $node, $val_type | | $($elem)* }
        build_implicit_leaf_struct! { $node, $leaf, $val_type }
        impl_implicit_node_trait! { $node, $leaf, $val_type, $($elem)* }
        impl_implicit_leaf_trait! { $node, $leaf, $val_type, $($elem)* }
        impl_implicit_size_trait! { $node, $leaf, $val_type, $($elem)* }
        impl_implicit_height_trait! { $node, $val_type, $($elem)* }
        impl_implicit_fold_trait! { $node, $val_type, $($elem)* }
    };
}


#[cfg(test)]
mod implicit_node_macro_test {
    use crate::data_structures::implicit_avl_tree::node_traits::*;
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

    def_implicit_node! { NodeTest, LeafTest, Am; size, height, fold, }

    #[test]
    fn node_macro_test() {
        let l = LeafTest::new(Am(10));
        let r = LeafTest::new(Am(100));
        let n = NodeTest::new(Child::Leaf(std::rc::Rc::new(std::cell::RefCell::new(l))), Child::Leaf(std::rc::Rc::new(std::cell::RefCell::new(r))));
        assert_eq!(n.size(), 2);
    }
}
