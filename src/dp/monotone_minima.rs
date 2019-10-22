use crate::dp::*;

use std::ops::Range;

fn monotone_minima_raw<F: Monotone>(f: &F, dp: &mut Vec<Option<(usize, F::Output)>>, xr: Range<usize>, yr: Range<usize>) {
    if xr.is_empty() { return; }
    let m = (xr.start + xr.end) / 2;
    let ans = None;
    for i in yr {
        let val = f.func(m, i);
        ans = match ans {
            None => Some((i, val)),
            Some((_, gr)) if val < gr => Some((i, val)),
            a => a,
        }
    }
    dp[m] = ans;
    let mi = ans.unwrap().0;
    monotone_minima_raw(f, dp, xr.start..m, yr.start..mi + 1);
    monotone_minima_raw(f, dp, m + 1..xr.end, mi..yr.end);
}

pub fn monotone_minima<F: Monotone>(f: &F) -> Vec<(usize, F::Output)> {
    let mut ans = vec![None; f.len()];
    monotone_minima_raw(f, &mut dp, 0..f.len(), 0..f.len());
    ans
}
