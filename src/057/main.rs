use std::time::Instant;

fn main() {
    let now = Instant::now();
    let (mut c, mut n, mut p, mut d) = (0, 1. as f64, 1. as f64, 2. as f64);
    for _ in 0..=1000 {
        (n, p, d) = (n * 2. + p, n, d + n);
        if n > 1e100 {
            (n, p, d) = (n / 1e10, p / 1e10, d / 1e10)
        }
        if n.log10().floor() > d.log10().floor() {
            c += 1;
        }
    }
    println!("Total count is {c}");
    let elapsed = now.elapsed();
    println!("Took {} mils", elapsed.as_millis());
}
