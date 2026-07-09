use std::time::Instant;

fn is_pentagon(num: usize) -> bool {
    let num_f = num as f32;
    let test = ((24.0 * num_f + 1.0).powf(0.5) + 1.0) / 6.0;
    test.fract() == 0.0
}

fn to_pentagon(n: usize) -> usize {
    (3 * n.pow(2) - n) / 2
}

fn sum_is_pentagon(x: usize, y: usize) -> bool {
    is_pentagon(x + y)
}

fn dif_is_pentagon(x: usize, y: usize) -> bool {
    is_pentagon(x - y)
}

fn resolve() {
    let mut x = 1;
    loop {
        for y in (1..x).rev() {
            let x_pent = to_pentagon(x);
            let y_pent = to_pentagon(y);
            if sum_is_pentagon(x_pent, y_pent) && dif_is_pentagon(x_pent, y_pent) {
                println!("{} pent num, {} pent num", x, y);
                println!("result {}, {}", x_pent, y_pent);
                println!("diff {}", x_pent - y_pent);
                return;
            }
        }
        x += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pentagon_01() {
        let c = to_pentagon(1);
        let e = 1;
        assert_eq!(c, e);
    }

    #[test]
    fn test_to_pentagon_02() {
        let c = to_pentagon(10);
        let e = 145;
        assert_eq!(c, e);
    }

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
    resolve();
    let elapsed = now.elapsed();
    println!("Took {} seconds", elapsed.as_secs());
}
