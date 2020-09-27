use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    for i in 1..s.len() {
        if s[i] == s[i - 1] {
            println!("{} {}", i, i + 1);
            return;
        }
        if i > 1 && s[i] == s[i - 2] {
            println!("{} {}", i - 1, i + 1);
            return;
        }
    }

    println!("-1 -1");
}
