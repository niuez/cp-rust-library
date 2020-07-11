pub mod directed_graph;
pub mod dijkstra;

pub trait Graph<'a> {
    type Edge: 'a;
    type DIter: Iterator<Item = &'a (usize, Self::Edge)>;
    fn vertices(&self) -> usize;
    fn delta(&'a self, v: &usize) -> Self::DIter;
}

