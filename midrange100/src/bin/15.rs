use itertools::Itertools;
use proconio::input;

fn fac(x: usize) -> usize {
    match x {
        1 => 1,
        _ => fac(x - 1) * x,
    }
}

fn dist(v1: (f64, f64), v2: (f64, f64)) -> f64 {
    ((v1.0 - v2.0) * (v1.0 - v2.0) + (v1.1 - v2.1) * (v1.1 - v2.1)).sqrt()
}

fn main() {
    input! {
        n: usize,
        xy:[(f64, f64); n]
    }

    let mut sum = 0f64;
    for idx in (0..n).permutations(n) {
        for i in 1..n {
            sum += dist(xy[idx[i]], xy[idx[i - 1]]);
        }
    }

    println!("{}", sum / (fac(n) as f64));
}
