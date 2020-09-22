use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        s: String,
    }

    let mut res = 0;
    for i in 0..s.len() {
        for j in i + 1..s.len() + 1 {
            let mut f = true;
            for k in i..j {
                f = f
                    && match &s.chars().nth(k).unwrap() {
                        'A' => true,
                        'T' => true,
                        'G' => true,
                        'C' => true,
                        _ => false,
                    }
            }
            if f {
                res = max(res, j - i);
            }
        }
    }
    println!("{}", res);
}
