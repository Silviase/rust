use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        h: usize,
        w: usize,
        senbei: [[i32; w]; h],
    }

    let mut res = 0;

    for bit in 0..(1 << h) {
        let mut cmp = 0;
        for col in 0..w {
            let mut tate = 0;
            for row in 0..h {
                if senbei[row][col] == (bit >> row) & 1 {
                    tate += 1;
                }
            }
            let plus = max(h - tate, tate);
            cmp += plus;
        }
        res = max(res, cmp);
    }
    println!("{}", res);
}
