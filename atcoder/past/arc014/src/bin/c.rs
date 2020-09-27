use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String,
    }

    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for ch in s.chars() {
        match ch {
            'R' => {
                r = 1 - r;
            }
            'G' => {
                g = 1 - g;
            }
            'B' => {
                b = 1 - b;
            }
            _ => {
                panic!();
            }
        }
    }

    println!("{}", r + g + b);
}
