use super::directed_tree::DirectedTree;

pub struct HeavyLightDecomposition {
    din: Vec<usize>,
    dout: Vec<usize>,
    sz: Vec<usize>,
    seq: Vec<usize>,
    heavy: Vec<Option<usize>>,
    next: Vec<Option<usize>>,
    par: Vec<Option<usize>>,
}

impl HeavyLightDecomposition {
    fn dfs_sz(&mut self, tree: &DirectedTree, v: usize) {
        self.sz[v] = 1;
        for &u in tree.next(v) {
            self.dfs_sz(tree, u);
            self.par[u] = Some(v);
            self.sz[v] += self.sz[u];
        }
        self.heavy[v] = tree.next(v).max_by_key(|&&i| self.sz[i]).map(|&x| x);
    }
    fn dfs_hld(&mut self, tree: &DirectedTree, v: usize, mut t: usize) -> usize {
        self.din[v] = t;
        self.seq.push(v);
        t = t + 1;
        if let Some(h) = self.heavy[v] {
            self.next[h] = self.next[v];
            t = self.dfs_hld(tree, h, t);
            for &u in tree.next(v).filter(|&&u| u != h) {
                self.next[u] = Some(u);
                t = self.dfs_hld(tree, u, t);
            }
        }
        self.dout[v] = t;
        t
    }
    pub fn build(tree: &DirectedTree) -> Self {
        let n = tree.len();
        let mut hld = HeavyLightDecomposition {
            din: vec![0; n],
            dout: vec![0; n],
            seq: Vec::new(),
            sz: vec![0; n],
            heavy: vec![None; n],
            next: vec![None; n],
            par: vec![None; n],
        };
        hld.dfs_sz(tree, tree.root());
        hld.dfs_hld(tree, tree.root(), 0);
        hld
    }
    pub fn sequence(&self) -> std::slice::Iter<usize> { self.seq.iter() }
    pub fn lca(&self, mut v: usize, mut u: usize) -> usize {
        loop {
            if self.din[u] > self.din[v] { std::mem::swap(&mut v, &mut u); }
            if self.next[u] == self.next[v] { break }
            v = self.par[self.next[v].unwrap()].unwrap();
        }
        u
    }
    pub fn path(&self, mut v: usize, mut u: usize, edge: bool) -> (Vec<std::ops::Range<usize>>, Vec<std::ops::Range<usize>>) {
        let mut l = Vec::new();
        let mut r = Vec::new();
        loop {
            if self.din[u] > self.din[v] {
                std::mem::swap(&mut v, &mut u);
                std::mem::swap(&mut l, &mut r);
            }
            if self.next[u] == self.next[v] {
                let e = if edge { 1 } else { 0 };
                l.push(self.din[u] + e..self.din[v] + 1);
                break
            }
            else {
                let ne = self.next[v].unwrap();
                l.push(self.din[ne]..self.din[v] + 1);
                v = self.par[ne].unwrap();
            }
        }
        (l, r)
    }
    pub fn subtree(&self, v: usize, edge: bool) -> std::ops::Range<usize> { self.din[v] + if edge { 1 } else { 0 }..self.dout[v] }
}
