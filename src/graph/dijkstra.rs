use std::collections::BinaryHeap;
use std::cmp::Reverse;

use super::Graph;

pub fn dijkstra<'a, G, W, EW>(g:&'a G, zero: W, inf: W, s: usize, weight: EW) -> Vec<(W, Option<usize>)>
where
    W: Clone + std::ops::Add<Output=W> + Ord,
    G: Graph<'a>,
    EW: Fn(&G::Edge) -> W,
{
    let mut dist = vec![(inf, None); g.vertices()];
    let mut que = BinaryHeap::new();
    dist[s] = (zero, None);
    que.push(Reverse((dist[s].0.clone(), s)));
    while let Some(Reverse((w, v))) = que.pop() {
        if dist[v].0 < w { continue }
        for (t, e) in g.delta(&v) {
            let cost = dist[v].0.clone() + weight(e);
            if dist[*t].0 > cost {
                dist[*t] = (cost, Some(v));
                que.push(Reverse((dist[*t].0.clone(), *t)));
            }
        }
    }
    dist
}
