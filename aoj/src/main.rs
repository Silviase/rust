use proconio::input;

use std::cmp::max;
fn main() {
    // Knapsack Problem
    input! {
        n:usize,
        lim:usize,
        vw: [(usize, usize); n],
    }

    let mut dp = vec![0 as usize; lim + 1];
    // dp[i] = {重さiで出来るものの最大値}

    for i in 0..n {
        for w in 0..lim + 1 {
            if w >= vw[i].1 {
                dp[w] = max(dp[w], dp[w - vw[i].1] + vw[i].0);
            }
        }
    }

    println!("{}", dp[lim])
}
