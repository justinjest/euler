use std::time::Instant;

fn gcd(i_one: usize, i_two: usize) -> usize {
    let mut x = i_one;
    let mut y = i_two;
    while y != 0 {
        let rem = x % y;
        x = y;
        y = rem;
    }
    x
}

fn euclid_formula(target: usize) -> usize {
    let mut count = 0;
    for m in 1_usize..target.isqrt() {
        for n in 1_usize..m {
            if gcd(m, n) == 1 && ((m + n) % 2 == 1) {
                let a = 2 * (m * n);
                let b = m.pow(2) - n.pow(2);
                let c = m.pow(2) + n.pow(2);
                let res = a + b + c;
                if target % res == 0 {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_01() {
        let check = gcd(12, 30);
        let expect = 6;
        assert_eq!(check, expect);
    }

    #[test]
    fn test_gcd_02() {
        let check = gcd(30, 12);
        let expect = 6;
        assert_eq!(check, expect);
    }

    #[test]
    fn num_possible_triangls_01() {
        let check = euclid_formula(120);
        let expect = 3;
        assert_eq!(check, expect);
    }
}

fn main() {
    let now = Instant::now();
    println!("Hello, world!");
    let mut largest = 0;
    for x in 2..1001 {
        let tmp = euclid_formula(x);
        if tmp > largest {
            println!("New largest: {}", x);
            largest = tmp;
        }
    }
    println!("Largest triangle found has {} solutions", largest);
    let elapsed = now.elapsed();
    println!("Took {} miliseconds", elapsed.as_millis());
}
