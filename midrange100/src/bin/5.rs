use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        a: usize,
        b: usize,
        ab: usize,
        mut x: usize,
        mut y: usize,
    }

    let mut res = 0;
    if a + b <= 2 * ab {
        res = a * x + b * y;
    } else {
        let mini = min(x, y);
        res += ab * 2 * mini;
        x -= mini;
        y -= mini;
        // println!("{}, {}", x, y);
        res += min(a, 2 * ab) * x + min(b, 2 * ab) * y;
    }
    println!("{}", res);
}
