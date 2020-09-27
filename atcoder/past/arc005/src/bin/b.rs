use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut y: i32,
        mut x: i32,
        dir: String,
        s: [Chars; 9],
    }

    x -= 1;
    y -= 1;
    let mut dx;
    let mut dy;

    if dir.contains("L") {
        dy = -1;
    } else if dir.contains("R") {
        dy = 1;
    } else {
        dy = 0;
    }
    if dir.contains("U") {
        dx = -1;
    } else if dir.contains("D") {
        dx = 1;
    } else {
        dx = 0;
    }

    for _ in 0..4 {
        print!("{}", s[x as usize][y as usize]);
        if x == 8 && dx == 1 {
            dx = -1;
        } else if x == 0 && dx == -1 {
            dx = 1;
        }
        if y == 8 && dy == 1 {
            dy = -1;
        } else if y == 0 && dy == -1 {
            dy = 1;
        }
        x += dx;
        y += dy;
    }
    println!("");
}
