use proconio::input;
use std::collections::HashSet;

fn dif(v1: (i64, i64), v2: (i64, i64)) -> (i64, i64) {
    (v2.0 - v1.0, v2.1 - v1.1)
}

fn add(v1: (i64, i64), v2: (i64, i64)) -> (i64, i64) {
    (v1.0 + v2.0, v1.1 + v2.1)
}

fn main() {
    input! {
        n:usize,
        search: [(i64, i64); n],
        m:usize,
        pics:[(i64, i64); m],
    }

    let mut pict = HashSet::new();
    for pic in pics {
        pict.insert(pic);
    }

    for fr in 0..search.len() {
        for to in pict.iter() {
            let diff = dif(search[fr], *to);
            let mut f = true;
            for i in 0..search.len() {
                f = f && pict.contains(&add(search[i], diff));
            }
            if f {
                println!("{} {}", diff.0, diff.1);
                return;
            }
        }
    }
}
