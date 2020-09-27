use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [Chars; n],
    }

    let mut res = vec![vec![0; m]; n];
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            res[i][j] = a[i - 1][j].to_digit(10).unwrap() - res[i - 1][j - 1] - res[i - 1][j + 1];
            if i > 1 {
                res[i][j] -= res[i - 2][j];
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            print!("{}", res[i][j]);
        }
        println!("");
    }
}
