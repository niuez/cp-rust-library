use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn dijkstra<W>(g: Vec<Vec<(usize, W)>>, zero: W, inf: W, s: usize) -> Vec<(W, Option<usize>)>
where
    W: Clone + std::ops::Add<Output=W> + Ord,
{
    let mut dist = vec![(inf, None); g.len()];
    let mut que = BinaryHeap::new();
    dist[s] = (zero, None);
    que.push(Reverse((dist[s].0.clone(), s)));
    while let Some(Reverse((w, v))) = que.pop() {
        if dist[v].0 < w { continue }
        for (t, cost) in g[v].iter() {
            if dist[*t].0 > dist[s].0.clone() + cost.clone() {
                dist[*t] = (dist[s].0.clone() + cost.clone(), Some(s));
                que.push(Reverse((dist[*t].0.clone(), *t)));
            }
        }
    }
    dist
}
