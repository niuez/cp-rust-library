use crate::dp::*;

use std::ops::Add;

pub fn knuth_yao_speedup<F: QuadrangleInequality>(init: &[F::Output], f: &F) -> Vec<Vec<Option<F::Output>>>
where F::Output: Add<F::Output, Output=F::Output> + Ord {
    let n = f.len();
    let mut dp = (0..n).map(|i| vec![None; n - i]).collect::<Vec<_>>();
    let mut k = (0..n).map(|i| vec![0; n - i]).collect::<Vec<_>>();


    for i in 0..n {
        dp[i][0] = Some(init[i]);
        k[i][0] = 0;
    }

    for d in 1..n {
        for i in 0..n-d {
            k[i][d] = (k[i][d - 1]..k[i + 1][d - 1] + if d == 1 { 1 } else { 2 }).min_by_key(|&s| { dp[i][s].unwrap() + dp[i + s + 1][d - s - 1].unwrap() }).unwrap();
            dp[i][d] = Some(dp[i][k[i][d]].unwrap() + dp[i + k[i][d] + 1][d - k[i][d] - 1].unwrap() + f.func(i..i + d + 1));
        }
    }
    dp
}
