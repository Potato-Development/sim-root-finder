fn main() {
    const CONST_THRESH: f64 = 0.000_1;
    const CONST_H: f64 = 0.000_000_000_000_1;

    let mut x: f64 = 50.0;
    let mut i: i32 = 0;

    while function(x) > CONST_THRESH && i < 100 {
        let y: f64 = function(x);
        let m: f64 = (function(x + CONST_H) - function(x - CONST_H)) / (2.0_f64 * CONST_H);
        let c: f64 = y - x * m;
        let x_next: f64 = -1.0 * (c / m);
        x = x_next;
        i += 1;
    }
    println!("Took {} iterations to find a root: x = {}.", i, x);
}

fn function(x: f64) -> f64 {
    // (x - 1)(x^5 + 3x^4 + 2x + 6) = x^6 + 2x^5 -3x^4 + 2x^2 + 4x -6
    // 1 is an obvious root of as (x - 1) is a factor of f(x)
    x.powi(6) + (2.0_f64 * x.powi(5)) - (3.0_f64 * x.powi(4))
        + (2.0_f64 * x.powi(2))
        + (4.0_f64 * x.powi(1))
        - (6.0_f64)
}
