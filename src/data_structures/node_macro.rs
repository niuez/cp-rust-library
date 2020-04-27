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
    ($node:ident, $val_type:ty | $($e:ident : $t:ty,)* | height, $($elem:tt)*) => {
        define_node! { $node, $val_type | $($e: $t,)* height: isize, | $($elem)* }
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
    ($node:ident, $val_type:ty | $($e:ident : $v:expr,)* | height, $($elem:tt)*) => {
        impl_node_elem! { $node, $val_type | $($e: $v,)* height: 1, | $($elem)* }
    };
    ($node:ident, $val_type:ty | $($e:ident : $v:expr,)* | fold, $($elem:tt)*) => {
        impl_node_elem! { $node, $val_type | $($e: $v,)* fold: <$val_type as Unital>::identity(), | $($elem)* }
    };
    ($node:ident, $val_type:ty | $($e:ident : $v:expr,)* | rev, $($elem:tt)*) => {
        impl_node_elem! { $node, $val_type | $($e: $v,)* rev: false, | $($elem)* }
    };
}

#[macro_export]
macro_rules! impl_node_trait {
    ($node:ident, $val_type:ty, $($elem:tt)* ) => {
        impl Node for $node {
            type Value = $val_type;
            fn push(&mut self) {
                impl_push! { self, $val_type, $($elem)* }
            }
            fn fix(&mut self) {
                impl_fix! { self, $val_type, $($elem)* }
            }
            fn child(&self, dir: usize) -> &Link<Self> { &self.child[dir] } 
            fn child_mut(&mut self, dir: usize) -> &mut Link<Self> { &mut self.child[dir] } 
            fn replace(&mut self, dir: usize, node: Link<Self>) -> Link<Self> {
                let nn = std::mem::replace(&mut self.child[dir], node);
                self.fix();
                nn
            }
            fn value(&self) -> &Self::Value { &self.val }
            fn value_mut(&mut self) -> &mut Self::Value { &mut self.val }
        }
    } 
}

#[macro_export]
macro_rules! impl_push {
    ($mself:expr, $val_type:ty, ) => {};
    ($mself:expr, $val_type:ty, rev, $($elem:tt)*) => {
        if $mself.rev {
            $mself.child.swap(0, 1);
            if let Some(ch) = $mself.child_mut(0).as_mut() {
                ch.reverse();
            }
            if let Some(ch) = $mself.child_mut(1).as_mut() {
                ch.reverse();
            }
        }
        $mself.rev = false;
        impl_push! { $mself, $val_type, $($elem)* }
    };
    ($mself:expr, $val_type:ty, $head:tt, $($elem:tt)*) => {
        impl_push! { $mself, $val_type, $($elem)* }
    };
}

#[macro_export]
macro_rules! impl_fix {
    ($mself:expr, $val_type:ty, ) => {};
    ($mself:expr, $val_type:ty, size, $($elem:tt)*) => {
        $mself.size = $mself.child[0].size() + $mself.child[1].size() + 1;
        impl_fix! { $mself, $val_type, $($elem)* }
    };
    ($mself:expr, $val_type:ty, height, $($elem:tt)*) => {
        $mself.height = std::cmp::max($mself.child[0].height(), $mself.child[1].height()) + 1;
        impl_fix! { $mself, $val_type, $($elem)* }
    };
    ($mself:expr, $val_type:ty, fold, $($elem:tt)*) => {
        $mself.fold = $mself.child[0].as_ref().map(|ch| ch.fold()).unwrap_or(<$val_type>::identity())
            .op(&$mself.val)
            .op(&$mself.child[1].as_ref().map(|ch| ch.fold()).unwrap_or(<$val_type>::identity()));
        impl_fix! { $mself, $val_type, $($elem)* }
    };
    ($mself:expr, $val_type:ty, $head:tt, $($elem:tt)*) => {
        impl_fix! { $mself, $val_type, $($elem)* }
    };
}

#[macro_export]
macro_rules! impl_rev_trait {
    ($node:ident, $val_type:ty, ) => {};
    ($node:ident, $val_type:ty, rev, $($tail:tt)*) => {
        impl ReversibleNode for $node {
            fn reverse(&mut self) {
                self.rev ^= true;
            }
        }
    };
    ($node:ident, $val_type:ty, $head:tt, $($elem:tt)*) => {
        impl_rev_trait! { $node, $val_type, $($elem)* }
    };
}
#[macro_export]
macro_rules! impl_rev2_trait {
    ($node:ident, $val_type:ty | off | off | ) => {};
    ($node:ident, $val_type:ty | off | on | ) => {};
    ($node:ident, $val_type:ty | on | off | ) => {
        impl ReversibleNode for $node {
            fn reverse(&mut self) {
                self.rev ^= true;
            }
        }
    };
    ($node:ident, $val_type:ty | on | on | ) => {
        impl ReversibleNode for $node {
            fn reverse(&mut self) {
                self.rev ^= true;
                self.fold = self.fold.reverse();
            }
        }
    };
    ($node:ident, $val_type:ty | $r:tt | $f:tt | rev, $($tail:tt)*) => {
        impl_rev2_trait! { $node, $val_type | on | $f | $($tail)* }
    };
    ($node:ident, $val_type:ty | $r:tt | $f:tt | fold, $($tail:tt)*) => {
        impl_rev2_trait! { $node, $val_type | $r | on | $($tail)* }
    };
    ($node:ident, $val_type:ty | $r:tt | $f:tt | $head:tt, $($tail:tt)*) => {
        impl_rev2_trait! { $node, $val_type | $r | $f | $($tail)* }
    };
}

#[macro_export]
macro_rules! impl_size_trait {
    ($node:ident, $val_type:ty, ) => {};
    ($node:ident, $val_type:ty, size, $($tail:tt)*) => {
        impl SizeNode for $node { fn size(&self) -> usize { self.size } }
    };
    ($node:ident, $val_type:ty, $head:tt, $($elem:tt)*) => {
        impl_size_trait! { $node, $val_type, $($elem)* }
    };
}

#[macro_export]
macro_rules! impl_height_trait {
    ($node:ident, $val_type:ty, ) => {};
    ($node:ident, $val_type:ty, height, $($tail:tt)*) => {
        impl HeightNode for $node { fn height(&self) -> isize { self.height } }
    };
    ($node:ident, $val_type:ty, $head:tt, $($elem:tt)*) => {
        impl_height_trait! { $node, $val_type, $($elem)* }
    };
}

#[macro_export]
macro_rules! impl_fold_trait {
    ($node:ident, $val_type:ty, ) => {};
    ($node:ident, $val_type:ty, fold, $($tail:tt)*) => {
        impl FoldNode for $node { fn fold(&self) -> $val_type { self.fold.clone() } }
    };
    ($node:ident, $val_type:ty, $head:tt, $($elem:tt)*) => {
        impl_fold_trait! { $node, $val_type, $($elem)* }
    };
}



#[macro_export]
macro_rules! def_node {
    ($node:ident, $val_type:ty; $($elem:tt)* ) => {
        define_node! { $node, $val_type | | $($elem)* }
        impl_node_elem! { $node, $val_type | | $($elem)* }
        impl_node_trait! { $node, $val_type, $($elem)* }
        impl_rev2_trait! { $node, $val_type | off | off | $($elem)* }
        impl_size_trait! { $node, $val_type, $($elem)* }
        impl_height_trait! { $node, $val_type, $($elem)* }
        impl_fold_trait! { $node, $val_type, $($elem)* }
    };
}

#[cfg(test)]
mod node_macro_test {
    use crate::data_structures::node_traits::*;

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
