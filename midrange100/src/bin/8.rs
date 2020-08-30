use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n:usize,
        ab: [(i64, i64); n],
    }

    let mut res_min = 1_000_000_000_000i64;
    for i in 0..ab.len() * 2 {
        for j in 0..ab.len() * 2 {
            let start = if i % 2 == 0 { ab[i / 2].0 } else { ab[i / 2].1 };
            let goal = if j % 2 == 0 { ab[j / 2].0 } else { ab[j / 2].1 };
            let mut cmp = 0;
            for k in 0..ab.len() {
                cmp += min(
                    (ab[k].0 - start).abs() + (ab[k].1 - ab[k].0).abs() + (goal - ab[k].1).abs(),
                    (ab[k].1 - start).abs() + (ab[k].0 - ab[k].1).abs() + (goal - ab[k].0).abs(),
                );
            }
            res_min = min(res_min, cmp);
        }
    }
    println!("{}", res_min);
}
