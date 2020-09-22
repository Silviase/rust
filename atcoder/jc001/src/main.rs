use proconio::*;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        ave: i64,
        x: [i64; n],
    }

    let mut array = vec![[0; 2501]; n];
    array[0][0] = 1;
    let mut res = 0i64;
    let mut p = 0i64;

    for i in 0..n {
        if x[i] == ave {
            p += 1;
            continue;
        }
        for j in x[i]..=2500 {
            array[j as usize] += array[(j - x[i]) as usize];
        }
    }

    println!("{}", res);
}
