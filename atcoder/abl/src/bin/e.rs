use ac_library_rs::lazysegtree;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
    }

    let mut lazy_seg = lazysegtree::LazySegtree::new(n);
    for i in 0..n {
        lazy_seg.set(i, 1);
    }

    for _ in 0..q {
        input! {
            l:usize,
            r:usize,
            d:usize,
        }
    }
}
