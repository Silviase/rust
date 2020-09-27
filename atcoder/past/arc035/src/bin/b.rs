use ac_library_rs::modint;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        mut t:[i64; n],
    }
    t.sort();

    let mut sum = t[0];
    let mut res = modint::ModInt1000000007::new(1);
    let mut fac_vec = vec![modint::ModInt1000000007::new(1); n + 1];
    for i in 1..=n {
        fac_vec[i] = fac_vec[i - 1] * modint::ModInt1000000007::new(i);
    }

    let mut stack = vec![];

    let mut len = 1;
    let mut acc = t[0];
    for i in 1..n {
        acc += t[i];
        sum += acc;
        if t[i] != t[i - 1] {
            stack.push(len);
            len = 1;
        } else {
            len += 1;
        }
    }
    stack.push(len);

    for e in stack {
        res *= fac_vec[e as usize];
    }

    println!("{}", sum);
    println!("{}", res);
}
