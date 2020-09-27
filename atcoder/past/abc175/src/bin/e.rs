use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
        hwv: [((usize, usize), usize); q],
    }

    let mut v = vec![vec![0; w + 1]; h + 1];
    let mut dp = vec![vec![vec![0; 4]; w + 1]; h + 1];

    for i in 0..q {
        v[(hwv[i].0).0][(hwv[i].0).1] += hwv[i].1;
    }

    for i in 1..=h {
        for j in 1..=w {
            for k in 0..=3 {
                if k == 0 {
                    dp[i][j][k] = max(dp[i][j - 1][0], dp[i - 1][j][3]);
                } else {
                    dp[i][j][k] = max(
                        max(dp[i][j - 1][k - 1] + v[i][j], dp[i - 1][j][3] + v[i][j]),
                        dp[i][j - 1][k],
                    );
                }
            }
        }
    }

    println!("{}", dp[h][w][3]);
}
