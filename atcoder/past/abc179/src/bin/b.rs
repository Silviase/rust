use proconio::*;

fn main() {
    input! {
        n:usize,
        d:[(i64, i64); n],
    }

    let mut f = false;
    for i in 0..n - 2 {
        if d[i].0 == d[i].1 && d[i + 1].0 == d[i + 1].1 && d[i + 2].0 == d[i + 2].1 {
            f = true;
            break;
        }
    }

    if f {
        println!("Yes");
    } else {
        println!("No");
    }
}
