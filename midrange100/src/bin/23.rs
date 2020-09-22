use proconio::{fastout, input};
use std::cmp::max;
#[fastout]
fn main() {
    input! {
        n: usize,
        lim: usize,
        mut p: [usize; n],
    }
    p.push(0);

    let mut p2 = vec![];
    for i in 0..=n {
        for j in 0..=n {
            p2.push(p[i] + p[j]);
        }
    }

    p2.sort();
    let nn = p2.len();

    let mut res = 0;
    for i in 0..nn {
        if lim < p2[i] {
            continue;
        }
        let target = lim - p2[i];
        let mut hi = nn;
        let mut lo = 0usize;
        while hi - lo > 1 {
            let mid = (hi + lo) / 2;
            if p2[mid] > target {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        res = max(res, p2[i] + p2[lo]);
    }
    println!("{}", res);
}
