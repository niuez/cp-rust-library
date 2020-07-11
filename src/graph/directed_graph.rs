use super::Graph;

pub struct DirectedGraph<E> {
    g: Vec<Vec<(usize, E)>>
}

impl<'a, E: 'a> Graph<'a> for DirectedGraph<E> {
    type Edge = E;
    type DIter = std::slice::Iter<'a, (usize, E)>;
    fn vertices(&self) -> usize {
        return self.g.len();
    }
    fn delta(&'a self, v: &usize) -> Self::DIter {
        return self.g[*v].iter();
    }
}

impl<E> DirectedGraph<E> {
    pub fn new(n: usize) -> Self {
        DirectedGraph { g: (0..n).map(|_| Vec::new()).collect() }
    }
    pub fn from_iter(n: usize, iter: impl IntoIterator<Item = (usize, usize, E)>) -> Self {
        let mut g = Self::new(n);
        for (f, t, e) in iter {
            g.add_edge(f, t, e);
        }
        g
    }
    pub fn add_edge(&mut self, from: usize, to: usize, e: E) {
        self.g[from].push((to, e));
    }
}
