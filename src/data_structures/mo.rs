pub trait MoState {
    type Elem;
    type Answer;
    fn new() -> Self;
    fn get_ans(&self) -> Self::Answer;
    fn add_right(&mut self, e: &Self::Elem);
    fn add_left(&mut self, e: &Self::Elem);
    fn delete_right(&mut self, e: &Self::Elem);
    fn delete_left(&mut self, e: &Self::Elem);
}

pub struct Mo<M: MoState> {
    q: usize,
    bsz: usize,
    qs: Vec<Vec<(usize, usize, usize)>>, 
    v: Vec<M::Elem>,
}

impl<M: MoState> Mo<M> {
    pub fn new<I>(n: usize, bucket_sz: usize, iter: I) -> Self 
    where
        I: IntoIterator<Item = M::Elem>
    {
        Self {
            q: 0,
            bsz: bucket_sz,
            qs: vec![Vec::new(); (n + bucket_sz - 1) / bucket_sz],
            v: iter.into_iter().collect(),
        }
    }

    pub fn add_query(&mut self, l: usize, r: usize) -> usize {
        let i = self.q;
        self.qs[l / self.bsz].push((r, l, i));
        self.q += 1;
        i
    }

    pub fn build(&mut self) {
        for s in 0..self.qs.len() {
            self.qs[s].sort();
        }
    }

    pub fn run(&self) -> Vec<Option<M::Answer>> {
        let mut ans: Vec<_> = (0..self.q).map(|_| None).collect();
        for s in 0..self.qs.len() {
            let mut state = M::new();
            let mut l = s * self.bsz;
            let mut r = l;
            for &(qr, ql, qi) in self.qs[s].iter() {
                while r < qr { state.add_right(&self.v[r]); r += 1; }
                while l > ql { l -= 1; state.add_left(&self.v[l]); }
                while r > qr { r-= 1; state.delete_right(&self.v[r]); }
                while l < ql { state.delete_left(&self.v[l]); l += 1; }
                ans[qi] = Some(state.get_ans());
            }
        }
        ans
    }

}
