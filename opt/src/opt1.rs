use gnuplot::*;

fn main() {
    let mut x = vec![];
    let mut y = vec![];
    let mut zero = vec![];

    let mut __now__ = 0.0;
    while __now__ < 3.0 {
        x.push(__now__);
        y.push(f(__now__));
        zero.push(0.0);
        __now__ += 0.001;
    }

    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&x, &y, &[Caption("y = 2x^2+6x-5"), Color("black")])
        .lines(&x, &zero, &[]);
    let _ = fg.save_to_png("production/images/f1.png", 300, 300);
}

fn f(x: f64) -> f64 {
    2.0 * x * x + 1.0 * x - 5.0
}
