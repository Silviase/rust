use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[Usize1; n],
        b:[i64; n],
        c:[i64; n-1],
    }

    let mut res = 0;
    for i in 0..n {
        res += b[a[i]];
        if i > 0 {
            if a[i - 1] + 1 == a[i] {
                res += c[a[i - 1]];
            }
        }
    }

    println!("{}", res);
}
