use gnuplot::{Caption, Color, Figure};

fn main() {
    let x = [0u32, 1, 2];
    let y = [3u32, 4, 5];
    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&x, &y, &[Caption("A line"), Color("black")]);
    let _ = fg.save_to_png("test.png", 300, 300);
}
