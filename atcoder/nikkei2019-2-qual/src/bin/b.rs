extern crate ac_library_rs;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize; n],
    }

    let mut cnt = vec![0; n];
    let mut res = ac_library_rs::ModInt998244353::new(1);

    for i in 0..n {
        if (i == 0) ^ (a[i] == 0) {
            println!("0");
            return;
        }
        cnt[a[i]] += 1;
    }

    for i in 1..n {
        res *= ac_library_rs::ModInt998244353::new(cnt[i - 1]).pow(cnt[i] as u64);
    }
    println!("{}", res);
}
