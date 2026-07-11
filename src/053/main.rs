use std::time::Instant;

static PI: f64 = 3.14159;
static E: f64 = 2.71828;

// This returns an approximation of n!, best used when numbers are not close
// to one.
fn stirling_approximation(n: f64) -> f64 {
    if n == 0.0 {
        return 1.0;
    }
    (2.0 * PI * n).powf(0.5) * (n / E).powf(n)
}

fn combinometric_approx(n: f64, r: f64) -> f64 {
    let top = stirling_approximation(n);
    let r_fac = stirling_approximation(r);
    let n_sub_r = stirling_approximation(n - r);

    top / (r_fac * n_sub_r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stirling_approximation_01() {
        let c = stirling_approximation(100.0);
        let e = 10.0;
        assert_eq!(c, e);
    }
}

fn main() {
    let now = Instant::now();
    let mut res = 0;
    for n in 1..=100 {
        for r in 0..=n {
            let c = combinometric_approx(n as f64, r as f64);
            if c > 1_000_000 as f64 {
                res += 1;
                println!("{}, {} find", n, res);
            }
        }
    }
    println!("{}", res);
    let elapsed = now.elapsed();
    println!("Took {} mils", elapsed.as_millis());
}
