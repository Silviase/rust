use proconio::{fastout, input};

#[fastout]
fn main() {
    let nine = 1_000_000_000f64;

    input! {
        n:usize,
        ar: [f64; n],
    }

    let mut two_five = [[0u64; 50]; 50];
    let mut res = 0;

    for i in 0..n {
        let mut x = (ar[i] * nine).round() as u64;
        let mut two = 0;
        let mut five = 0;
        while x % 2 == 0 {
            x /= 2;
            two += 1;
        }
        while x % 5 == 0 {
            x /= 5;
            five += 1;
        }
        for i in 0..50 {
            for j in 0..50 {
                if i + two >= 18 && j + five >= 18 {
                    res += two_five[i][j];
                }
            }
        }
        two_five[two][five] += 1;
    }

    println!("{}", res);

    // 高々-10 ~ 32の間に収まるはず
}
