mod parse_commands;

use num::Complex;

// GENERIC struct example
struct ComplexNumberType<T> {
    re: T,
    im: T
}

/// Try to determine if `c` is in the mandelbrot set, using at most `limit`.
/// The 3 slashes are used to write documentation in Rust.

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize>{
    let mut z = Complex { re: 0.0, im: 0.0 };
    // Iterate from `0` to limit, when limit is reached, then it'll stop
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i)
        }
        z = z * z + c;
    }
    None
}

fn square_loop(mut c: f64){
    let mut x: f64 = 0.0;
    loop {
        x = (x * x) + c;
    }
}

fn complex_square_add_loop(c: Complex<f64>){
    let mut z = Complex {re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}


fn main() {
    println!("Hello, world!");
}
