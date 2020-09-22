use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        m: usize,
        ar:[[usize; m]; n],
    }

    let mut res = 0;
    for i in 0..m {
        for j in i + 1..m {
            let mut team_score = 0;
            for k in 0..n {
                team_score += max(ar[k][i], ar[k][j]);
            }
            // println!("{} , ({}, {})", team_score, i, j);
            res = max(res, team_score);
        }
    }
    println!("{}", res);
}
