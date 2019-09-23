use std::rc::Rc;
use std::cell::RefCell;


enum StreamRaw<T> {
    Nil,
    Cons(Rc<T>, Stream<T>, usize),
    Concat(Stream<T>, Stream<T>, usize),
    Reverse(Stream<T>, usize),
}

impl<T> Clone for StreamRaw<T> {
    fn clone(&self) -> Self {
        match *self {
            StreamRaw::Nil => StreamRaw::Nil,
            StreamRaw::Cons(ref x, ref s, len) => StreamRaw::Cons(x.clone(), s.clone(), len),
            StreamRaw::Concat(ref f, ref r, len) => StreamRaw::Concat(f.clone(), r.clone(), len),
            StreamRaw::Reverse(ref r, len) => StreamRaw::Reverse(r.clone(), len),
        }
    }
}

pub struct Stream<T> {
    st: Rc<RefCell<StreamRaw<T>>>,
}

impl<T> Clone for Stream<T> {
    fn clone(&self) -> Self {
        Stream { st: self.st.clone() }
    }
}

impl<T> Stream<T> {
    pub fn new() -> Self {
        Stream { st: Rc::new(RefCell::new(StreamRaw::Nil)) }
    }
    pub fn len(&self) -> usize {
        let st = &*self.st.borrow();
        match *st {
            StreamRaw::Nil => 0,
            StreamRaw::Cons(_, _, len) => len,
            StreamRaw::Concat(_, _, len) => len,
            StreamRaw::Reverse(_, len) => len,
        }
    }
    pub fn eval(&self) {
        let st = &mut *self.st.borrow_mut();
        let result = if let StreamRaw::Concat(ref front, ref rear, len) = *st {
            //println!("concat");
            front.eval();
            match *front.st.borrow() {
                StreamRaw::Nil => { rear.eval(); rear.st.borrow().clone() },
                StreamRaw::Cons(ref x, ref s, _) => {
                    StreamRaw::Cons(x.clone(), Stream { st: Rc::new(RefCell::new(StreamRaw::Concat(s.clone(), rear.clone(), len - 1))) }, len)
                }
                _ => unreachable!(),
            }
        }
        else if let StreamRaw::Reverse(ref stream, _) = *st {
            //println!("reversing");
            let mut now = stream.clone();
            let mut r = StreamRaw::Nil;
            let mut rlen = 0;
            while let &StreamRaw::Cons(ref x, ref s, _) = {
                now.eval();
                &*now.clone().st.borrow()
            } {
                r = StreamRaw::Cons(x.clone(), Stream { st: Rc::new(RefCell::new(r)) }, rlen);
                rlen += 1;
                now = s.clone();
            }
            r
        }
        else {
            return;
        };
        *st = result;
    }
    pub fn cons(&self, x: T) -> Self {
        self.eval();
        Stream { st: Rc::new(RefCell::new(StreamRaw::Cons(Rc::new(x), self.clone(), self.len() + 1))) }
    }

    pub fn head(&self) -> Option<Rc<T>> {
        self.eval();
        let st = &*self.st.borrow();
        match *st {
            StreamRaw::Nil => None,
            StreamRaw::Cons(ref x, _, _) => Some(x.clone()),
            _ => unreachable!(),
        }
    }
    pub fn tail(&self) -> Stream<T> {
        self.eval();
        let st = &*self.st.borrow();
        match *st {
            StreamRaw::Nil => Stream::new(),
            StreamRaw::Cons(_, ref s, _) => s.clone(),
            _ => unreachable!(),
        }
    }
    pub fn concat(&self, other: Self) -> Self {
        Stream { st: Rc::new(RefCell::new(StreamRaw::Concat(self.clone(), other.clone(), self.len() + other.len()))) }
    }
    pub fn reverse(&self) -> Self {
        //println!("will reverse");
        Stream { st: Rc::new(RefCell::new(StreamRaw::Reverse(self.clone(), self.len()))) }
    }
}

pub struct Queue<T> {
    front: Stream<T>,
    rear: Stream<T>
}

impl<T> Clone for Queue<T> {
    fn clone(&self) -> Self {
        Queue { front: self.front.clone(), rear: self.rear.clone() }
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { front: Stream::new(), rear: Stream::new() }
    }

    fn check(front: Stream<T>, rear: Stream<T>) -> Self {
        if rear.len() <= front.len() {
            Queue { front: front, rear: rear }
        }
        else {
            Queue { front: front.concat(rear.reverse()), rear: Stream::new() }
        }
    }

    pub fn snoc(&self, x: T) -> Self {
        Queue::check(self.front.clone(), self.rear.cons(x))
    }

    pub fn head(&self) -> Option<Rc<T>> {
        self.front.head()
    }

    pub fn tail(&self) -> Self {
        Queue::check(self.front.tail(), self.rear.clone())
    }
}

#[test]
fn bankers_queue_test() {
    {
        println!("simple ----");
        let que = Queue::new();
        let que = que.snoc(1);
        let que = que.snoc(2);
        let que = que.snoc(3);
        let que2 = que.clone();
        let que = que.snoc(4);
        println!("{:?}", que.head());
        let que = que.tail();
        println!("{:?}", que.head());
        let que = que.tail();
        println!("{:?}", que.head());
        let que = que.tail();
        println!("{:?}", que.head());
        let que = que.tail();
        println!("{:?}", que.head());

        let que = que2.snoc(10);
        let que = que.snoc(9);
        let que = que.snoc(8);
        println!("{:?}", que.head());
        let que = que.tail();
        println!("{:?}", que.head());
        let que = que.tail();
        println!("{:?}", que.head());
        let que = que.tail();
        println!("{:?}", que.head());
        let que = que.tail();
        println!("{:?}", que.head());
        let que = que.tail();
        println!("{:?}", que.head());
        let que = que.tail();
        println!("{:?}", que.head());
        let que = que.tail();
        println!("{:?}", que.head());
        let que = que.tail();
        println!("{:?}", que.head());
    }
    {
        println!("15 =========");
        let mut que = Queue::new();
        for i in 1..16 {
            println!("{}==", i);
            que = que.snoc(i);
        }
        let que2 = que.clone();
        for i in 1..16 {
            println!("{}==", i);
            println!("{:?}", que.head());
            que = que.tail();
        }
        let mut que = que2.snoc(16);
        println!("16=========");
        for i in 1..17 {
            println!("{}==", i);
            println!("{:?}", que.head());
            que = que.tail();
        }
    }
}
