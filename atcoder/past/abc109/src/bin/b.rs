use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n:usize,
        s:[Chars; n],
    }

    let mut f = true;
    let mut hs = HashSet::new();

    for i in 0..n - 1 {
        let si: String = s[i].iter().collect();
        if hs.contains(&si) {
            f = false;
        }
        if s[i][s[i].len() - 1] != s[i + 1][0] {
            f = false;
            break;
        }
        hs.insert(si);
    }

    if f {
        println!("Yes");
    } else {
        println!("No");
    }
}
