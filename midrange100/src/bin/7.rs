use proconio::input;
use std::cmp::max;
use std::collections::HashSet;

fn main() {
    input! {
        n:usize,
        xy: [(i32, i32); n],
    }

    let mut hs = HashSet::new();
    for i in 0..xy.len() {
        hs.insert(xy[i]);
    }

    let mut res = 0;
    for i in 0..xy.len() {
        for j in 0..xy.len() {
            if i == j {
                continue;
            }
            let vector = (xy[j].0 - xy[i].0, xy[j].1 - xy[i].1);
            let v3 = add(xy[j], hodge(vector));
            let v4 = add(v3, hodge(hodge(vector)));
            if hs.contains(&v3) && hs.contains(&v4) {
                // println!("{:?}, {:?}, {:?}, {:?}", xy[i], xy[j], v3, v4);
                res = max(res, abs_square(vector));
            }
        }
    }
    println!("{}", res);
}

fn hodge(v: (i32, i32)) -> (i32, i32) {
    (-v.1, v.0)
}

fn add(v1: (i32, i32), v2: (i32, i32)) -> (i32, i32) {
    (v1.0 + v2.0, v1.1 + v2.1)
}

fn abs_square(v: (i32, i32)) -> i32 {
    v.0 * v.0 + v.1 * v.1
}
