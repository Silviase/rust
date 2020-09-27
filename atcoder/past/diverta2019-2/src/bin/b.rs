use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n:usize,
        mut xy:[(i64,i64); n],
    }

    xy.sort_by_key(|t| t.0);
    let mut hm = HashMap::new();
    for i in 0..n {
        for j in i + 1..n {
            let dif = (xy[j].0 - xy[i].0, xy[j].1 - xy[i].1);
            let v = hm.get(&dif).unwrap_or(1);
            if hm.contains_key(&dif) {
                hm.insert(&dif, v);
            }
        }
    }
    todo!();
}
