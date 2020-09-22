use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        p:[Usize1; n],
        q:[Usize1; n],
    }

    let mut cp = 0i32;
    let mut cq = 0i32;
    let mut cnt = 0;
    for idx in (0..n).permutations(n) {
        cnt += 1;
        // println!("{:?}", idx);
        // println!("p:{:?}", p);
        // println!("q:{:?}", q);
        let mut eqp = true;
        let mut eqq = true;
        for i in 0..n {
            eqp = eqp && idx[i] == p[i];
            eqq = eqq && idx[i] == q[i];
        }
        if eqp {
            cp = cnt;
        }
        if eqq {
            cq = cnt;
        }
    }
    println!("{}", (cq - cp).abs());
}
