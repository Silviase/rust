use proconio::input;
use proconio::marker::Usize1;
use std::cmp::max;
use std::collections::HashSet;

fn main() {
    input! {
        h:usize,
        w:usize,
        m:usize,
        xy:[(Usize1, Usize1); m],
    }

    let mut x = vec![0i64; h];
    let mut y = vec![0i64; w];
    let mut max_x = 0;
    let mut max_y = 0;
    let mut hs = HashSet::new();

    for i in 0..m {
        x[xy[i].0] += 1;
        y[xy[i].1] += 1;
        hs.insert(xy[i]);
        max_x = max(max_x, x[xy[i].0]);
        max_y = max(max_y, y[xy[i].1]);
    }

    let mut bomb = true;

    let mut vx = vec![];
    let mut vy = vec![];
    for i in 0..h {
        if x[i] == max_x {
            vx.push(i);
        }
    }
    for i in 0..w {
        if y[i] == max_y {
            vy.push(i);
        }
    }

    for hei in vx {
        for w in 0..vy.len() {
            if !hs.contains(&(hei, vy[w])) {
                bomb = false;
                break;
            }
        }
    }

    let mut res = max_x + max_y;
    if bomb {
        res -= 1;
    }
    println!("{}", res);
}
