use crate::data_structures::containers::rerooting_persistent_array::RerootingPersistentArray;

use std::cell::RefCell;

pub struct ConchonFilliatrePersistentUnionFind {
    rank: RerootingPersistentArray<u32>,
    par: RefCell<RerootingPersistentArray<usize>>,
}

impl Clone for ConchonFilliatrePersistentUnionFind {
    fn clone(&self) -> Self { Self { rank: self.rank.clone(), par: self.par.clone() }}
}

impl ConchonFilliatrePersistentUnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            rank: RerootingPersistentArray::from_iter(std::iter::repeat(0).take(n)),
            par: RefCell::new(RerootingPersistentArray::from_iter(0..n)),
        }
    }

    pub fn root(&self, v: usize) -> usize {
        let p = *self.par.borrow().get(v);
        if p == v {
            p
        }
        else {
            let r = self.root(p);
            self.par.replace_with(|p| p.set(v, r));
            r
        }
    }
    
    pub fn union(&self, u: usize, v: usize) -> Self {
        let u = self.root(u);
        let v = self.root(v);
        if u == v {
            self.clone()
        }
        else {
            let ru = *self.rank.get(u);
            let rv = *self.rank.get(v);
            match ru.cmp(&rv) {
                std::cmp::Ordering::Greater => Self {
                    rank: self.rank.clone(),
                    par: RefCell::new(self.par.borrow().set(v, u)),
                },
                std::cmp::Ordering::Equal => Self {
                    rank: self.rank.set(u, ru + 1u32),
                    par: RefCell::new(self.par.borrow().set(v, u)),
                },
                std::cmp::Ordering::Less => Self {
                    rank: self.rank.clone(),
                    par: RefCell::new(self.par.borrow().set(u, v)),
                }
            }
        }
    }
}
