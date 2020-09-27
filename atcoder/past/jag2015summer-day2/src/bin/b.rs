use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        k:i64,
    }

    let mut res = 0i64;
    for _ in 0..n - 1 {
        res += res / (k - 1) + 1;
    }
    println!("{}", res);
}
