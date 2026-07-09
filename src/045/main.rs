use std::time::Instant;

fn is_pentagon(num: usize) -> bool {
    let num_f = num as f64;
    let test = ((24.0 * num_f + 1.0).powf(0.5) + 1.0) / 6.0;
    test.fract() == 0.0
}

fn is_hexagon(num: usize) -> bool {
    let num_f = num as f64;
    let test = ((8.0 * num_f + 1.0).powf(0.5) + 1.0) / 4.0;
    test.fract() == 0.0
}

fn to_triangle(n: usize) -> usize {
    n * (n + 1) / 2
}

fn resolve(starting: usize) {
    let mut x = starting;
    loop {
        let t = to_triangle(x);
        if is_hexagon(t) && is_pentagon(t) {
            println!("{} is all three", t);
            return;
        }
        x += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pentagon_01() {
        let c = is_pentagon(1);
        let e = true;
        assert_eq!(c, e);
    }

    #[test]
    fn test_is_pentagon_02() {
        let c = is_pentagon(2);
        let e = false;
        assert_eq!(c, e);
    }

    #[test]
    fn test_is_pentagon_03() {
        let c = is_pentagon(34);
        let e = false;
        assert_eq!(c, e);
    }
}

fn main() {
    let now = Instant::now();
    resolve(40756);
    let elapsed = now.elapsed();
    println!("Took {} seconds", elapsed.as_secs());
}
