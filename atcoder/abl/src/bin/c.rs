use ac_library_rs::Dsu;
use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
    }

    let mut uf = Dsu::new(n);
    let mut con = vec![false; n];
    for edge in xy {
        uf.merge(edge.0, edge.1);
    }

    for i in 0..n {
        con[uf.leader(i)] = true;
    }

    let mut res = -1;
    for i in 0..n {
        if con[i] {
            res += 1;
        }
    }
    println!("{}", res);
}
