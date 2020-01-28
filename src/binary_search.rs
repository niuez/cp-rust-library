use std::cmp::Ordering;

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut ng = -1i64;
        let mut ok = self.len() as i64;
        while ok - ng > 1 {
            let m = (ok + ng) / 2;
            match self[m as usize].cmp(x) {
                Ordering::Greater | Ordering::Equal => {
                    ok = m;
                }
                _ => {
                    ng = m;
                }
            }
        }
        ok as usize
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut ng = -1i64;
        let mut ok = self.len() as i64;
        while ok - ng > 1 {
            let m = (ok + ng) / 2;
            match self[m as usize].cmp(x) {
                Ordering::Greater => {
                    ok = m;
                }
                _ => {
                    ng = m;
                }
            }
        }
        ok as usize
    }
}
