use ac_library_rs::Dsu;
use proconio::{fastout, input, marker::Usize1};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        q:[(Usize1, Usize1); n],
    }

    let mut xy = q.clone();
    xy.sort_by_key(|v| v.0);

    let mut uf = Dsu::new(n);
    let mut hs = HashSet::new();
    hs.insert(&xy[0]);
    for i in 1..n {
        // addはy座標の最小
        let mut add = (n + 1, n + 1);
        for e in hs.clone().iter() {
            // より大きい場合
            if e.1 < xy[i].1 {
                // println!("merge {:?} <-> {:?}", xy[i], vec[j]);
                uf.merge(i, e.0);
                if add.0 == n + 1 {
                    add = **e;
                }
            }
        }

        if add > n {
            hs.insert(&xy[i]);
        }
    }

    for i in 0..n {
        println!("{}", uf.size(q[i].0));
    }
}
