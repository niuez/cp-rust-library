pub struct DirectedTree {
    n: usize,
    r: usize,
    g: Vec<Vec<usize>>,
    p: Vec<Option<(usize, usize)>>,
}

impl DirectedTree {
    pub fn new<I: IntoIterator<Item=(usize, usize)>>(n: usize, es: I) -> Self {
        let mut g: Vec<_> = (0..n).map(|_| Vec::new()).collect();
        let mut p = vec![None; n];
        for (u, v) in es {
            p[v] = Some((u, g[u].len()));
            g[u].push(v);
        }
        DirectedTree {
            n: n,
            r: (0..n).find(|&x| p[x].is_none()).unwrap(),
            g: g,
            p: p,
        }
    }

    pub fn next(&self, v: usize) -> std::slice::Iter<usize> { self.g[v].iter() }
    pub fn parent(&self, v: usize) -> Option<usize> { self.p[v].map(|(u, i)| self.g[u][i]) }
    pub fn root(&self) -> usize { self.r }
    pub fn len(&self) -> usize { self.n }
}
