use crate::math::modint::inv_mod;

pub fn garner<XI>(xm: XI, m: u32) -> u32
where
    XI: IntoIterator<Item=(u32, u32)>, 
{
    let mut xm = xm.into_iter().map(|(x, y)| (x as u64, y as u64)).collect::<Vec<_>>();
    xm.push((0, m as u64));
    let mut c = vec![(1u64, 0u64); xm.len()];
    for i in 0..xm.len() - 1 {
        let a = xm[i].0 + xm[i].1 - c[i].1;
        let a = if a >= xm[i].1 { a - xm[i].1 } else { a };
        let v = a * inv_mod(c[i].0, xm[i].1) % xm[i].1;
        for j in i + 1..xm.len() {
            c[j].1 = (c[j].1 + c[j].0 * v) % xm[j].1;
            c[j].0 = (c[j].0 * xm[i].1) % xm[j].1
        }
    }
    c[xm.len() - 1].1 as u32
}
