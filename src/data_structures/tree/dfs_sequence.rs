use crate::data_structures::tree::directed_tree::DirectedTree;

use std::ops::Range;

pub struct DfsSequence {
    seq: Vec<usize>,
    ran: Vec<Range<usize>>,
}

impl DfsSequence {
    fn build_dfs<T>(mut self, tree: &DirectedTree<T>, v: usize) -> Self {
        let i = self.seq.len();
        self.seq.push(v);
        for &(w, _) in tree.next(v) {
            self = self.build_dfs(tree, w);
        }
        self.ran[v] = i..self.seq.len();
        self
    }
    pub fn build<T>(tree: &DirectedTree<T>) -> Self {
        let n = tree.len();
        let et = DfsSequence { seq: Vec::new(), ran: vec![n..n; n] };
        et.build_dfs(tree, tree.root())
    }
    pub fn sequence(&self) -> &[usize] { &self.seq }
    pub fn range(&self, v: usize) -> Range<usize> { self.ran[v].clone() }
}

