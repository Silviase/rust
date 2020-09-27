use ac_library_rs::modint;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut dp = vec![vec![modint::ModInt1000000007::new(0); 13]; s.len() + 1];
    dp[0][0] += modint::ModInt1000000007::new(1);
    for i in 0..s.len() {
        if s[i] == '?' {
            for j in 0..13 {
                for k in 0..=9 {
                    dp[i + 1][(j * 10 + k) % 13] = dp[i + 1][(j * 10 + k) % 13] + dp[i][j];
                }
            }
        } else {
            let digit = s[i].to_digit(10).unwrap() as usize;
            for j in 0..13 {
                dp[i + 1][(j * 10 + digit) % 13] = dp[i + 1][(j * 10 + digit) % 13] + dp[i][j];
            }
        }
    }

    println!("{}", dp[s.len()][5]);
}
