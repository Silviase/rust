use proconio::input;
fn main() {
    input! {
        n:isize,
        mut ar: [usize; n]
    }

    ar.sort();
    let mut num = [0; 1_000_001];
    for x in ar {
        num[x] += 1;
    }

    let mut res = 0;

    for i in 1..=1_000_000 {
        if num[i] == 0 {
            continue;
        }
        if num[i] == 1 {
            res += 1;
        }

        for j in (0..=1_000_000).step_by(i) {
            num[j] = 0;
        }
    }
    println!("{}", res);
}
