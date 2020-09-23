use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }
    let PI = std::f64::consts::PI;
    let lx = a * (2f64 * PI / 60f64 * m).cos();
    let ly = a * (2f64 * PI / 60f64 * m).sin();
    let sx = b * (2f64 * PI / 720f64 * (h * 60f64 + m)).cos();
    let sy = b * (2f64 * PI / 720f64 * (h * 60f64 + m)).sin();

    let res = ((lx - sx) * (lx - sx) + (ly - sy) * (ly - sy)).sqrt();

    println!("{}", res);
}
