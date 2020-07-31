use super::{ Line, LineNumber };

pub struct LiChaoSegmentTree<T> {
    node: Box<[Option<Line<T>>]>,
    xs: Box<[T]>,
    sz: usize,
}

impl<T: LineNumber> LiChaoSegmentTree<T> {
    pub fn init(xs: &[T]) -> Self {
        let sz = xs.len().next_power_of_two();
        let mut xs = xs.to_vec();
        xs.resize(sz, *xs.last().unwrap());
        Self {
            node: vec![None; sz << 1].into_boxed_slice(),
            xs: xs.into_boxed_slice(),
            sz,
        }
    }

    fn update_range(&mut self, mut i: usize, mut l: usize, mut r: usize, mut line: Line<T>) {
        while i < (self.sz << 1) {
            if let Some(li) = self.node[i].take() {
                let m = (l + r) >> 1;
                let bl = line.get(self.xs[l]) < li.get(self.xs[l]);
                let bm = line.get(self.xs[m]) < li.get(self.xs[m]);
                if bm { 
                    self.node[i] = Some(std::mem::replace(&mut line, li));
                }
                else {
                    self.node[i] = Some(li)
                }
                if bl != bm {
                    r = m;
                    i = i << 1;
                }
                else {
                    l = m;
                    i = (i << 1) + 1;
                }
            }
            else {
                self.node[i] = Some(line);
                break
            };
        }
    }

    pub fn add_line(&mut self, line: Line<T>) {
        self.update_range(1, 0, self.sz, line);
    }

    pub fn add_segment(&mut self, l: usize, r: usize, line: Line<T>) {
        let mut left = l;
        let mut right = r;
        let mut l = l + self.sz;
        let mut r = r + self.sz;
        let mut len = 1;
        while l < r {
            if (l & 1) == 1 {
                self.update_range(l, left, left + len, line.clone());
                l += 1;
                left += len;
            }
            if (r & 1) == 1 {
                r -= 1;
                self.update_range(r, right - len, right, line.clone());
                right -= len;
            }
            l = l >> 1;
            r = r >> 1;
            len = len << 1;
        }
    }

    pub fn get_min(&self, i: usize) -> Option<T> {
        let x = self.xs[i];
        let mut i = i + self.sz;
        let mut ans = None;
        while i > 0 {
            let res = self.node[i].as_ref().map(|l| l.get(x));
            ans = match (ans, res) {
                (Some(a), Some(b)) if a < b => Some(a),
                (Some(_), Some(b)) => Some(b),
                (Some(a), _) => Some(a),
                (None, b) => b,
            };
            i = i >> 1;
        }
        ans
    }
}
