use proconio::input;

fn main() {
    input! {
        n:usize,
        mut hs:[(usize, usize); n],
    }

    let mut lo = 0usize;
    let mut hi = 1_000_000_000_000_000usize;
    while hi - lo > 1 {
        let mid = (hi + lo) / 2;
        if is_ok(mid, &hs, n) {
            hi = mid;
        } else {
            lo = mid;
        }
        // println!("hi:{}, lo:{}", hi, lo);
    }
    println!("{}", hi);
}

fn is_ok(target: usize, array: &[(usize, usize)], n: usize) -> bool {
    let mut ika = vec![0; n];
    for i in 0..n {
        if target < array[i].0 {
            return false;
        }
        let t = (target - array[i].0) / array[i].1;
        if t < n {
            ika[t] += 1;
        }
    }

    for i in 0..n {
        if i == 0 {
            if ika[i] > 1 {
                return false;
            }
        } else {
            ika[i] = ika[i] + ika[i - 1];
            if ika[i] > i + 1 {
                return false;
            }
        }
    }

    true
}
