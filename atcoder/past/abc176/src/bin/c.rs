use proconio::*;
fn main() {
    input! {
        n :usize,
        ar : [i64; n],
    }

    let mut forward = 0i64;
    let mut res = 0;
    for person in ar {
        if person > forward {
            forward = person;
        } else {
            res += forward - person;
        }
    }

    println!("{}", res);
}
