use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,
        s:Chars,
    }

    let mut res = 0;
    for i in 0..1000 {
        let pass = vec![i / 10 / 10, i / 10 % 10, i % 10 % 10];
        let mut done = 0;
        for j in 0..n {
            if s[j] as usize - 48 == pass[done] {
                done += 1;
                if done == 3 {
                    res += 1;
                    break;
                }
            }
        }
    }

    println!("{}", res);
}
