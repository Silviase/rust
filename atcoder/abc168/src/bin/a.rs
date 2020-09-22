use proconio::input;

fn main() {
    input! {
        n:i32,
    }
    match n % 10 {
        2 => println!("hon"),
        4 => println!("hon"),
        5 => println!("hon"),
        7 => println!("hon"),
        9 => println!("hon"),
        0 => println!("pon"),
        1 => println!("pon"),
        6 => println!("pon"),
        8 => println!("pon"),
        3 => println!("bon"),
        _ => unreachable!(),
    }
}
