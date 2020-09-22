fn hodge(v: (T, T)) -> (T, T) {
    (-v.1, v.0)
}

fn add(v1: (T, T), v2: (T, T)) -> (T, T) {
    (v1.0 + v2.0, v1.1 + v2.1)
}

fn dif(v1: (T, T), v2: (T, T)) -> (T, T) {
    (v2.0 - v1.0, v2.1 - v1.0)
}

fn abs_square(v: (T, T)) -> T {
    v.0 * v.0 + v.1 * v.1
}
