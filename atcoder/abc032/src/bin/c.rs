use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: [usize; n],
    }

    // 末尾がiで積がk以下であるようなものを探す.
    // 先頭をfとすると, f~i, f+1~i,,,iなのでi-f+1通り足し上げればいい

    for i in 0..n {
        if s[i] == 0 {
            println!("{}", n);
            return;
        }
    }

    let mut first = 0;
    let mut prod = 1usize;

    let mut res = 0;
    for last in 0..n {
        prod *= s[last];
        while prod > k {
            if first > last {
                break;
            }
            prod /= s[first];
            first += 1;
        }
        if prod <= k {
            res = max(res, last + 1 - first);
        }
    }
    println!("{}", res);
}
