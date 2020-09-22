use proconio::*;
use std::cmp::max;

fn main() {
    input! {
        a:i64,
        b:i64,
        c:i64,
        d:i64,
    }
    println!("{}", max(a * c, max(b * d, max(a * d, b * c))));
}
