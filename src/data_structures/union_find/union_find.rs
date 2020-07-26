use std::cell::RefCell;

pub struct UnionFind {
    par: RefCell<Vec<usize>>,
    sz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            par: RefCell::new((0..n).collect()),
            sz: std::iter::repeat(1).take(n).collect(),
        }
    }

    pub fn root(&self, v: usize) -> usize {
        let p = self.par.borrow()[v];
        if p == v { v }
        else {
            let r = self.root(p);
            self.par.borrow_mut()[v] = r;
            r
        }
    }

    pub fn unite(&mut self, a: usize, b: usize) -> Option<(usize, usize)> {
        let a = self.root(a);
        let b = self.root(b);
        if a == b { None }
        else if self.sz[a] < self.sz[b] {
            self.par.borrow_mut()[a] = b;
            self.sz[b] += self.sz[a];
            Some((b, a))
        }
        else {
            self.par.borrow_mut()[b] = a;
            self.sz[a] += self.sz[b];
            Some((a, b))
        }
    }
}
