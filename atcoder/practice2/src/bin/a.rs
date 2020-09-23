extern crate ac_library_rs;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        tuv: [[usize; 3]; q],
    }

    let mut uf = ac_library_rs::Dsu::new(n);

    for i in 0..q {
        if tuv[i][0] == 0 {
            uf.merge(tuv[i][1], tuv[i][2]);
        } else {
            if uf.same(tuv[i][1], tuv[i][2]) {
                println!("1");
            } else {
                println!("0");
            }
        }
    }
}
