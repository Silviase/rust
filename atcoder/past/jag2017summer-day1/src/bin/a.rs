use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        mut n: usize,
    }
    if n <= 52 {
        println!("{}", n / 2 * 3 + n % 2);
        return;
    }

    let v = vec![
        0usize, 26, 676, 17576, 456976, 11881376, 308915776, 8031810176,
    ];
    let mut res = 0usize;
    let mut i = 1usize;
    while n > 0 {
        res += min(n * i, v[i] * i);
        n -= min(n, v[i]);
        i += 1;
    }
    println!("{}", res);
}
