use proconio::*;
use std::cmp::max;

fn main() {
    input! {
        n:usize,
        xy:[(i64, i64); n],
    }

    let mut sum = Vec::new();
    let mut dif = Vec::new();
    for e in xy {
        sum.push(e.0 + e.1);
        dif.push(e.0 - e.1);
    }
    sum.sort();
    dif.sort();

    println!("{}", max(sum[n - 1] - sum[0], dif[n - 1] - dif[0]));
}
