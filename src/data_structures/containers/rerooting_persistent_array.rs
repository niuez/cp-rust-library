use std::rc::Rc;
use std::cell::{ Ref, RefCell };

enum Node<T> {
    Arr(Box<[T]>),
    Diff {
        idx: usize,
        val: T,
        base: RerootingPersistentArray<T>,
    }
}

pub struct RerootingPersistentArray<T> {
    node: Rc<RefCell<Node<T>>>,
}


impl<T> Clone for RerootingPersistentArray<T> {
    fn clone(&self) -> Self {
        Self { node: self.node.clone() }
    }
}

impl<T> RerootingPersistentArray<T> {
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item=T>,
    {
        Self { node: Rc::new(RefCell::new(
                    Node::Arr(iter.into_iter().collect::<Vec<_>>().into_boxed_slice())
                    )), }
    }

    fn reroot(&self) {
        let node = &mut *self.node.borrow_mut();
        if let &mut Node::Diff { idx, ref mut val, ref mut base } = node {
            let base = std::mem::replace(base, self.clone());
            base.reroot();
            let base_node = &mut *base.node.borrow_mut();
            match base_node {
                &mut Node::Arr(ref mut arr) => {
                    std::mem::swap(&mut arr[idx], val);
                }
                _ => unreachable!(),
            }
            std::mem::swap(base_node, node);
        }
    }

    pub fn get(&self, i: usize) -> Ref<'_, T> {
        self.reroot();
        Ref::map(self.node.borrow(), |node| match node {
            &Node::Arr(ref arr) => &arr[i],
            _ => unreachable!(),
        })
    }

    pub fn set(&self, idx: usize, val: T) -> Self {
        Self { node: Rc::new(RefCell::new(
                    Node::Diff { idx, val, base: self.clone() }
                    )) }
    }
}
