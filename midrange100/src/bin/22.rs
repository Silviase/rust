use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        p: f64,
    }
    let mut lo = 0f64;
    let mut hi = p;
    while hi - lo > 1e-12 {
        let mid = (lo + hi) / 2f64;
        if f(mid, p) < 0f64 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    println!("{}", h(lo, p));
}

fn f(x: f64, p: f64) -> f64 {
    1f64 - p * 2f64 * 2f64.ln() / 3f64 * 2f64.powf(-x / 1.5)
}

fn h(n: f64, p: f64) -> f64 {
    n + p * 2f64.powf(-n / 1.5)
}
