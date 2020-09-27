use ac_library_rs::modint;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut fac = vec![modint::ModInt998244353::new(1); n + 1];
    for i in 1..n {
        fac[i] = fac[i - 1] * modint::ModInt998244353::new(i);
    }

    let mut res = modint::ModInt998244353::new(0);
    for t in 0..=k {
        res += fac[n - 1] / fac[n - 1 - t] / fac[t]
            * modint::ModInt998244353::new(m - 1).pow((n - t - 1) as u64)
            * modint::ModInt998244353::new(m);
    }
    println!("{}", res);
}
