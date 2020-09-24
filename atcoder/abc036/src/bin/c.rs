use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut elements = a.clone();
    elements.sort();
    let mut idx = 0;
    let mut map = HashMap::new();
    for i in 0..n {
        let e = elements[i];
        if !map.contains_key(&e) {
            map.insert(e, idx);
            idx += 1;
        }
    }

    for i in 0..n {
        println!("{}", map.get(&a[i]).unwrap());
    }
}
