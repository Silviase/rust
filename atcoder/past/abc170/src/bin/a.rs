use proconio::*;

fn main() {
    input! {
        x: [i32; 5]
    }
    for i in 0..5 {
        if x[i] - 1 != i as i32 {
            println!("{}", i + 1);
        }
    }
}
