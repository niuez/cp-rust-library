use crate::data_structures::tree::directed_tree::DirectedTree;

use std::collections::VecDeque;
pub struct VertexEulerTour {
    pub arr: Vec<usize>,
    pub idx: Vec<usize>,
}

#[derive(Clone, Copy)]
pub enum RedBlack {
    Red,
    Black,
}

impl VertexEulerTour {
    pub fn build(tree: &DirectedTree<RedBlack>) -> Self {
        let n = tree.len();
        let mut arr = Vec::new();
        let mut idx = vec![0; n];

        let mut deq = VecDeque::new();
        deq.push_front((tree.root(), tree.root(), None));
        while let Some((v, r, op)) = deq.pop_front() {
            idx[v] = arr.len();
            arr.push(v);
            for &(w, color) in tree.next(v) {
                match color {
                    RedBlack::Red => deq.push_back((w, w, Some(r))),
                    RedBlack::Black => deq.push_front((w, r, op)),
                }
            }
        }
        VertexEulerTour { arr: arr, idx: idx }
    }
}

#[test]
fn euler_tour_test() {
    let g = DirectedTree::new(15, (1..15).map(|x| {
        ((x - 1) / 2, x, if x % 2 == 1 { RedBlack::Red } else { RedBlack::Black })
    }));
    let et = VertexEulerTour::build(&g);
    println!("{:?}", et.arr);
    println!("{:?}", et.idx);
}
