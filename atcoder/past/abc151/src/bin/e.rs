use ac_library_rs::modint;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i64; n],
    }
    a.sort();
    let up = a.clone();
    a.sort_by_key(|t| -t);
    let down = a.clone();
    let mut fac = vec![modint::ModInt1000000007::new(1)];
    for i in 1..=n {
        fac.push(*fac.get(i - 1).unwrap() * modint::ModInt1000000007::new(i));
    }

    let mut min_x = modint::ModInt1000000007::new(0);
    let mut max_x = modint::ModInt1000000007::new(0);
    for i in 0..=n - k {
        min_x +=
            modint::ModInt1000000007::new(up[i]) * fac[n - 1 - i] / fac[k - 1] / fac[n - k - i];
        max_x +=
            modint::ModInt1000000007::new(down[i]) * fac[n - 1 - i] / fac[k - 1] / fac[n - k - i];
    }

    println!("{}", max_x - min_x);
}
