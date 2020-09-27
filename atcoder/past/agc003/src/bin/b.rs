use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        mut a:[i64; n],
    }
    let mut res = 0;
    for i in 0..n {
        res += a[i] / 2;
        a[i] %= 2;
        if i < n - 1 {
            if a[i] > 0 && a[i + 1] > 0 {
                a[i + 1] -= 1;
                res += 1;
            }
        }
    }

    println!("{}", res);
}
