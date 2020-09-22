use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize; n],
        mut b:[usize; n],
        mut c:[usize; n],
    }

    a.sort();
    b.sort();
    c.sort();

    // 各bについて考えればいいことがわかる
    let mut res = 0;
    for cmp in b {
        let sma;
        let big;

        let mut lo = -1i32;
        let mut hi = n as i32;
        while hi - lo > 1 {
            let mid = ((hi + lo) / 2) as usize;
            if a[mid as usize] >= cmp {
                hi = mid as i32;
            } else {
                lo = mid as i32;
            }
        }
        sma = hi as usize;

        lo = -1;
        hi = n as i32;
        while hi - lo > 1 {
            let mid = ((hi + lo) / 2) as usize;
            if c[mid] > cmp {
                hi = mid as i32;
            } else {
                lo = mid as i32;
            }
        }
        big = n - hi as usize;
        res += sma * big;
    }
    println!("{}", res);
}
