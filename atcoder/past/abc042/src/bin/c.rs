use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: i64,
        k: usize,
    }
    let mut hs = HashSet::new();
    for _ in 0..k {
        input! {
            hate: i64,
        }
        hs.insert(hate);
    }

    let mut res = n;
    loop {
        let mut f = true;
        let mut check = res.clone();

        while check > 0 {
            // 全て含んでいない必要がある
            f &= !hs.contains(&(check % 10i64));
            check /= 10;
        }

        if f {
            break;
        }
        res += 1;
    }

    println!("{}", res);
}
