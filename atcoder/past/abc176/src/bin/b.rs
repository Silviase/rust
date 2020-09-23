use proconio::*;

fn main() {
    input! {
        s : String
    }
    let mut a = 0;
    for ch in s.as_str().chars() {
        a = (a + (ch as i32) - ('0' as i32)) % 9;
    }
    let f = if a == 0 { "Yes" } else { "No" };
    println!("{}", f);
}
