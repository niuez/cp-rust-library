use crate::data_structures::node_traits::*;
use crate::algebra::*;

pub struct ArrFoldNode<T: Monoid> {
    val: T,
    fold: T,
    size: usize,
    child: [Link<ArrFoldNode<T>>; 2],
    rev: bool,
} 



impl<T: Monoid> ArrFoldNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val: val,
            fold: T::identity(),
            size: 1,
            child: [ None, None ],
            rev: false,
        }
    }
}

impl<T: Monoid> Node for ArrFoldNode<T> {
    type Value = T;
    fn push(&mut self) {
        if self.rev {
            self.child.swap(0, 1);
        }
        self.rev = false
    }
    fn fix(&mut self) {
        self.size = size(&self.child[0]) + size(&self.child[1]) + 1;
        self.fold = fold(&self.child[0]).op(&self.val).op(&fold(&self.child[1]));
    }
    fn child(&self, dir: usize) -> &Link<Self> { &self.child[dir] } 
    fn child_mut(&mut self, dir: usize) -> &mut Link<Self> { &mut self.child[dir] } 
    fn take(&mut self, dir: usize) -> Link<Self> {
        let nn = self.child[dir].take();
        self.fix();
        nn
    }
    fn set(&mut self, dir: usize, node: Link<Self> ) {
        self.child[dir] = node;
        self.fix();
    }
    fn value(&self) -> &Self::Value { &self.val }
    fn value_mut(&mut self) -> &mut Self::Value { &mut self.val }
}

impl<T: Monoid> ReversibleNode for ArrFoldNode<T> {
    fn reverse(&mut self) {
        self.rev ^= true;
        //reversing
    } 
}

impl<T: Monoid> SizeNode for ArrFoldNode<T> {
    fn size(&self) -> usize { self.size }
}

impl<T: Monoid> FoldNode for ArrFoldNode<T> {
    fn fold(&self) -> T { self.fold.clone() } 
}


#[macro_export]
macro_rules! build_node_struct {
    ($node:ident, $val_type:ty, $($elem:ident: $t:ty,)*) => {
        struct $node {
            val: $val_type,
            child: [Link<$node>; 2],
            $($elem: $t),*
        }
    }
}

#[macro_export]
macro_rules! define_node {
    ($node:ident, $val_type:ty | $($e:ident : $t:ty,)* | ) => {
        build_node_struct! { $node, $val_type, $($e: $t,)* }
    };
    ($node:ident, $val_type:ty | $($e:ident : $t:ty,)* | size, $($elem:tt)*) => {
        define_node! { $node, $val_type | $($e: $t,)* size: usize, | $($elem)* }
    };
    ($node:ident, $val_type:ty | $($e:ident : $t:ty,)* | fold, $($elem:tt)*) => {
        define_node! { $node, $val_type | $($e: $t,)* fold: $val_type, | $($elem)* }
    };
    ($node:ident, $val_type:ty | $($e:ident : $t:ty,)* | rev, $($elem:tt)*) => {
        define_node! { $node, $val_type | $($e: $t,)* rev: bool, | $($elem)* }
    };
}


#[macro_export]
macro_rules! impl_node_new {
    ($node:ident, $val_type:ty, $($e:ident : $v: expr,)*) => {
        impl $node {
            fn new(val: $val_type) -> Self {
                Self {
                    val: val,
                    child: [None, None],
                    $($e: $v),*
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_node_elem {
    ($node:ident, $val_type:ty | $($e:ident : $v:expr,)* | ) => {
        impl_node_new! { $node, $val_type, $($e: $v,)* }
    };
    ($node:ident, $val_type:ty | $($e:ident : $v:expr,)* | size, $($elem:tt)*) => {
        impl_node_elem! { $node, $val_type | $($e: $v,)* size: 1, | $($elem)* }
    };
    ($node:ident, $val_type:ty | $($e:ident : $v:expr,)* | fold, $($elem:tt)*) => {
        impl_node_elem! { $node, $val_type | $($e: $v,)* fold: $val_type::identity(), | $($elem)* }
    };
    ($node:ident, $val_type:ty | $($e:ident : $v:expr,)* | rev, $($elem:tt)*) => {
        impl_node_elem! { $node, $val_type | $($e: $v,)* rev: false, | $($elem)* }
    };
}


#[macro_export]
macro_rules! def_node {
    ($node:ident, $val_type:ty; $($elem:tt)* ) => {
        define_node! { $node, $val_type | | $($elem)* }
        impl_node_elem! { $node, $val_type | | $($elem)* }
    };
}

#[cfg(test)]
mod node_macro_test {
    use super::*;

    struct M(usize);
    def_node! { NodeTest, M; size, rev, }
    
    #[test]
    fn node_macro_test() {
        let n = NodeTest { val: M(91), child: [None, None], size: 10, rev: false };
        assert_eq!(n.val.0, 91);
        assert_eq!(n.size, 10);
        let n = NodeTest::new(M(15));
        assert_eq!(n.val.0, 15);
        assert_eq!(n.size, 1);
        assert_eq!(n.rev, false);
    }
}
