use std::rc::Rc;

pub struct Heap<T: Ord> {
    rank: usize,
    elem: Rc<T>,
    left: LeftistHeap<T>,
    right: LeftistHeap<T>,
}

impl<T: Ord> Heap<T> {
    fn new(x: Rc<T>) -> Self {
        Heap {
            rank: 1,
            elem: x,
            left: LeftistHeap::None,
            right: LeftistHeap::None,
        }
    }
    fn meld(l: Rc<Self>, r: &LeftistHeap<T>) -> Self {
        let right = LeftistHeap::meld(&l.as_ref().right, &r);
        let left = l.as_ref().left.clone();
        if left.rank() >= right.rank() {
            Heap {
                rank: right.rank() + 1,
                elem: l.as_ref().elem.clone(),
                left: left,
                right: right,
            }
        }
        else {
            Heap {
                rank: left.rank() + 1,
                elem: l.as_ref().elem.clone(),
                left: right,
                right: left,
            }
        }
    }
    fn peek(&self) -> &T { &self.elem.as_ref() }
    fn pop(&self) -> LeftistHeap<T> {
        LeftistHeap::meld(&self.left, &self.right)
    }
}

pub enum LeftistHeap<T: Ord> {
    None,
    Some(Rc<Heap<T>>),
}

impl<T: Ord> Clone for LeftistHeap<T> {
    fn clone(&self) -> Self {
        match self {
            &LeftistHeap::None => LeftistHeap::None,
            &LeftistHeap::Some(ref heap) => LeftistHeap::Some(heap.clone()),
        }
    }
}

impl<T: Ord> LeftistHeap<T> {
    pub fn new() -> Self {
        LeftistHeap::None
    }
    pub fn singleton(x: T) -> Self {
        LeftistHeap::Some(Rc::new(Heap::new(Rc::new(x))))
    }
    fn rank(&self) -> usize {
        match self {
            &LeftistHeap::None => 0,
            &LeftistHeap::Some(ref heap) => heap.as_ref().rank,
        }
    }
    pub fn meld(a: &Self, b: &Self) -> Self {
        match a {
            &LeftistHeap::None => b.clone(),
            &LeftistHeap::Some(ref ah) => match b {
                &LeftistHeap::None => a.clone(),
                &LeftistHeap::Some(ref bh) => 
                    if ah.as_ref().peek() >= bh.as_ref().peek() {
                        LeftistHeap::Some(Rc::new(Heap::meld(ah.clone(), b)))
                    }
                    else {
                        LeftistHeap::Some(Rc::new(Heap::meld(bh.clone(), a)))
                    }
            }
        }
    }
    pub fn peek(&self) -> Option<&T> {
        match self {
            &LeftistHeap::None => None,
            &LeftistHeap::Some(ref heap) => Some(heap.as_ref().peek()),
        }
    }
    pub fn insert(&self, x: T) -> Self {
        LeftistHeap::meld(self, &LeftistHeap::singleton(x))
    }
    pub fn pop(&self) -> Self {
        match self {
            &LeftistHeap::None => unreachable!("can't pop an empty heap"),
            &LeftistHeap::Some(ref heap) => heap.as_ref().pop(),
        }
    }
}

#[test]
fn leftist_heap_test() {
    let mut vec = Vec::new();
    vec.push(LeftistHeap::new());

    let h = vec[0].insert(0);
    vec.push(h);
    assert_eq!(vec[0].peek(), None);
    assert_eq!(vec[1].peek(), Some(&0));

    let h = vec[1].insert(1);
    vec.push(h);
    assert_eq!(vec[2].peek(), Some(&1));

    let h = vec[2].insert(2);
    vec.push(h);
    assert_eq!(vec[3].peek(), Some(&2));

    let h = vec[2].insert(1);
    assert_eq!(h.peek(), Some(&1));
    let h = h.pop();
    assert_eq!(h.peek(), Some(&1));
    let h = h.pop();
    assert_eq!(h.peek(), Some(&0));
    let h = h.pop();
    assert_eq!(h.peek(), None);
}
