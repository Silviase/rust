use proconio::{fastout, input, marker::Chars};
use std::collections::HashMap;

#[fastout]
fn main() {
    let mut char_to_ord = HashMap::new();
    let alp = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
    for i in 0..26 {
        char_to_ord.insert(alp.get(i), i);
    }
    input! {
        s: Chars,
        t: Chars,
    }
    let mut alp_idx = vec![vec![]; 26];
    for i in 0..s.len() {
        let idx = *char_to_ord.get(&s.get(i)).unwrap();
        alp_idx[idx].push(i);
    }
    for i in 0..s.len() {
        let idx = *char_to_ord.get(&s.get(i)).unwrap();
        alp_idx[idx].push(i + s.len());
    }

    let mut res = -1i64;
    let mut roop = 0;

    for i in 0..t.len() {
        let idx = *char_to_ord.get(&t.get(i)).unwrap();
        let mut lo = 0;
        let mut hi = alp_idx[idx].len();
        if alp_idx[idx].len() == 0 {
            println!("-1");
            return;
        }
        while hi - lo > 0 {
            let mid = (hi + lo) / 2;
            if alp_idx[idx][mid] as i64 <= res {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        res = alp_idx[idx][hi] as i64;
        // println!("res : {}", res);
        if res as usize >= s.len() {
            res -= s.len() as i64;
            roop += 1;
        }
    }

    // println!("{:?}", alp_idx);
    println!("{}", roop * s.len() + res as usize + 1);
}
