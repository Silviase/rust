use proconio::*;

fn main() {
    input! {
        x:i32,
        n:isize,
        ar:[i32; n],
    }
    let mut dif = 0;
    loop {
        let minus = x - dif;
        let plus = x + dif;
        let mut f = false;
        for e in 0..ar.len() {
            if ar[e] == minus {
                f = true;
            }
        }
        if !f {
            println!("{}", minus);
            return;
        }

        f = false;
        for e in 0..ar.len() {
            if ar[e] == plus {
                f = true;
            }
        }
        if !f {
            println!("{}", plus);
            return;
        }

        dif += 1;
    }
    // unreachable!();
}
