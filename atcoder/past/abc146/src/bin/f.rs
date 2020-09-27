use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut n: usize,
        m: usize,
        s: Chars,
    }

    let mut idx = n;
    let mut stack = vec![];

    while idx > 0 {
        let mut can = false;
        for i in (1..=m).rev() {
            let nxt = idx as i32 - i as i32;

            if nxt >= 0 {
                // println!("{}", s[nxt as usize]);

                if s[nxt as usize] == '0' {
                    can = true;
                    idx = nxt as usize;
                    stack.push(i);
                    break;
                }
            }
        }

        if !can {
            println!("-1");
            return;
        }
    }

    while !stack.is_empty() {
        print!("{} ", stack.pop().unwrap());
    }
    println!("");
}
