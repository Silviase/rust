use proconio::input;

fn main() {
    input! {
        n :usize,
    }

    let mut res = 0;
    for i in 1..=n {
        if yakusu(i) == 8 {
            res += 1;
        }
    }
    println!("{}", res);
}

fn yakusu(x: usize) -> usize {
    let mut res = 0;
    for i in (1..=x).step_by(2) {
        if x % i == 0 {
            res += 1;
        }
    }

    res
}
