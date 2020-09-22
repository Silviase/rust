use proconio::input;
use proconio::marker::Usize1;
use std::cmp::max;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut hs = HashSet::new();
    for _ in 0..m {
        input! {
            p:(Usize1, Usize1),
        }
        hs.insert(p);
    }

    let mut res = 0;
    for bit in 0..1 << n {
        let mut using = vec![false; n];
        let mut num = 0;
        for i in 0..n {
            if bit & (1 << i) > 0 {
                using[i] = true;
                num += 1;
            }
        }

        let mut f = true;
        for i in 0..n {
            for j in i + 1..n {
                if !using[i] || !using[j] {
                    continue;
                }
                // i, jともに用いるはず
                f = f && hs.contains(&(i, j));
            }
        }
        if f {
            res = max(res, num);
        }
    }
    println!("{}", res);
}
