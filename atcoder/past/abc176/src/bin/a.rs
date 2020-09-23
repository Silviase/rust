use proconio::input;

fn main() {
    input! {
        n: i32,
        x: i32,
        t: i32,
    }
    let ans = (n + x - 1) / x * t;
    println!("{}", ans);
}
