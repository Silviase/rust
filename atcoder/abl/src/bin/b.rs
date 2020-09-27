use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:i64,
        b:i64,
        c:i64,
        d:i64,
    }
    if b <= d {
        if c <= b {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if a <= d {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
