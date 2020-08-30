use proconio::input;

fn main() {
    input! {
        n: usize,
        elements: [usize; n],
        q: usize,
        targets :[usize; q],
    }

    for target in targets {
        let mut f = false;
        for bit in 0..1 << n {
            let mut sum = 0;
            for x in 0..n {
                sum += match bit & (1 << x) {
                    0 => 0,
                    _ => elements[x],
                }
            }
            if sum == target {
                f = true;
                break;
            }
        }
        if f {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
