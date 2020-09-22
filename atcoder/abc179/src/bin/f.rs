use proconio::*;

fn main() {
    input! {
        n:i64,
        q:usize,
        query:[(usize, usize); q],
    }

    let mut fixed_tate = vec![false; (n - 2) as usize];
    let mut fixed_yoko = vec![false; (n - 2) as usize];
    let mut tate = vec![n - 2; (n - 2) as usize];
    let mut yoko = vec![n - 2; (n - 2) as usize];
    let mut black = (n - 2) * (n - 2);

    for qw in query {
        if qw.0 == 1 {
            // tate
            black -= tate[qw.1 - 2];

            // tate はfixされる
            for i in qw.1 - 2..fixed_tate.len() {
                if fixed_tate[i] {
                    break;
                }
                fixed_tate[i] = true;
            }
        } else {
        }
    }

    println!("{}", res);
}
