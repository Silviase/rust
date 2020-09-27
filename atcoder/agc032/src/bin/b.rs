use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    // kは必ずn以下の最大の偶数になる
    let mut v = vec![];
    let k = n / 2 * 2;
    for i in 1..=k {
        for j in i + 1..=k {
            if i + j != k + 1 {
                v.push((i, j));
            }
        }
    }

    if n != k {
        for i in 1..n {
            v.push((i, n));
        }
    }

    println!("{}", v.len());
    for e in v {
        println!("{} {}", e.0, e.1);
    }
}
