use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        k: usize,
        height: [usize; n],
    }
    let mut res: usize = 1_000_000_000_000;

    for bit in 0..1 << n {
        let mut col: usize = 0;
        let mut budget: usize = 0;
        let mut now_h: usize = 0;
        for i in 0..n {
            if now_h < height[i] {
                now_h = height[i];
                col += 1;
            } else if bit & (1 << i) > 0 {
                now_h += 1;
                budget += now_h - height[i];
                col += 1;
            }
        }
        if col >= k {
            res = min(res, budget);
        }
    }

    println!("{}", res);
}
