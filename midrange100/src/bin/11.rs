use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut vaild: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..m {
        input! {
            k:usize,
            switches:[Usize1; k],
        }
        vaild.insert(i, switches.into_iter().collect::<Vec<usize>>());
    }
    input! {
        amari:[usize; m],
    }

    let mut res = 0;
    for bit in 0..1 << n {
        let mut on = vec![0; n];
        for i in 0..n {
            on[i] = bit & (1 << i);
        }
        let mut f = true;
        for i in 0..m {
            let mut sum = 0;
            for j in vaild.get(&i).unwrap() {
                sum += if on[*j] > 0 { 1 } else { 0 };
            }
            if (sum % 2) != amari[i] {
                f = false;
                break;
            }
        }

        if f {
            res += 1;
        }
    }
    println!("{}", res);
}
