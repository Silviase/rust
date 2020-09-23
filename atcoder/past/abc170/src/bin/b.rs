use proconio::*;
fn main() {
    input! {
        x:i32,
        y:i32,
    }
    let a = (4 * x - y) / 2;
    let b = (-2 * x + y) / 2;
    if a >= 0 && b >= 0 && a + b == x && 2 * a + 4 * b == y {
        println!("Yes");
    } else {
        println!("No");
    }
}
