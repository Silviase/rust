use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:i32,
        b:i32,
    }
    let res = ((b - 1) + (a - 2)) / (a - 1);
    println!("{}", res);
}
