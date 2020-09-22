use proconio::*;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        s: usize,
    }

    let mut array = vec![0i64; 2001];
    array[0] = 1;
    for i in 3..=2000 {
        for j in 3..=i {
            array[i] = (array[i] + array[i - j]) % MOD;
        }
    }
    println!("{}", array[s]);
}
