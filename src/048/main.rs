use std::time::Instant;

fn modular_pow(base: u64, exponent: u64, modulo: u64) -> u64 {
    if modulo == 1 {
        return 0;
    }
    let mut c = 1;
    for _ in 0..exponent {
        c = (c * base) % modulo;
    }
    return c;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modular_pow_01() {
        let e = modular_pow(4, 13, 497);
        let c = 445;
        assert_eq!(e, c);
    }

    #[test]
    fn modular_pow_02() {
        let e = modular_pow(1000, 1000, 1_000_000_000);
        let c = 0;
        assert_eq!(e, c);
    }

    #[test]
    fn sum_modular_pow() {
        let mut e = 0;
        for i in 1..=10 {
            let tmp = modular_pow(i, i, 10 * 10_u64.pow(9));
            println!("{}", tmp);
            e += tmp;
        }
        let c = 405071317;
        assert_eq!(e, c);
    }
}

fn main() {
    println!("{}", std::mem::size_of::<usize>());
    let now = Instant::now();
    let mut ans = 0;
    for i in 1..=1000 {
        ans = (ans + modular_pow(i, i, 10_000_000_000)) % 10_000_000_000;
    }
    println!("After {:04} we have the digits {:010}", 1000, ans);
    let elapsed = now.elapsed();
    println!("Took {} mils", elapsed.as_millis());
}
