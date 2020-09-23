use proconio::*;
fn main() {
    input! {
        n:i64,
    }
    let mut res = 0i64;
    for i in 1..n {
        if n % i == 0 {
            res += n / i - 1;
        } else {
            res += n / i;
        }
    }
    println!("{}", res);
}
