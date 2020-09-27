use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m:usize,
        v:usize,
        p:usize,
        mut a:[i64; n],
    }

    a.sort_by_key(|t| -t);
    let mut acc = vec![0i64; n + 1];
    for i in 1..=n {
        acc[i] = acc[i - 1] + a[i - 1];
    }

    let mut lo = 0;
    let mut hi = n;
    while hi - lo > 0 {
        let mid = (hi + lo) / 2;
        let f;
        if mid < p {
            f = true;
        } else if a[mid] + (m as i64) < a[p - 1] {
            f = false;
        } else {
            let mut vote = m * (p - 1) + m * (n - mid);
            for i in p - 1..mid {
                vote += a[mid] as usize + m - a[i] as usize;
            }
            if vote < m * v {
                f = false;
            } else {
                f = true;
            }
        }

        if f {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    println!("{}", hi);
}
