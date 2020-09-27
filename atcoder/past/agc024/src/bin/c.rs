use ac_library_rs::{segtree::Max, Segtree};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize; n],
    }

    let mut segtree = ac_library_rs::Segtree::<Max<_>>::new(n);
    for i in 0..n {
        segtree.set(i, a[i]);
    }
}
