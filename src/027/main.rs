use std::time::Instant;

fn is_prime(n: i32) -> bool {
    if n < 0 {
        return false;
    }
    for i in 2..n.isqrt()+1 {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn next_prime(n: i32) -> i32 {
    let next_num = n + 1;
    for i in 2..=next_num.isqrt() {
        if next_num % i == 0 {
            return next_prime(next_num);
        }
    }
    return next_num;
}

fn calculate_num_primes(b: i32, c: i32) -> i32 {
   // This will return the number of primes for consecutive values
   // Starting at 0, using the formual n^2 + b*n + c
   // For 1, 41 this should return 39
    let mut num_primes = 0;
    let mut n: i32 = 0;
    loop {
       let result: i32 = n.pow(2) + b*n + c;
       if is_prime(result) {
           num_primes += 1;
           n += 1;
       } else {
           return n;
       }
   }
}

fn main() {
    let now = Instant::now();
    let mut c = 1; // This is the starting point for C which must be prime
    let mut most_primes = 0;
    let mut highest_b = 0;
    let mut highest_c = 0;
    for b in -1000..1000 {
        while c < 1000 {
            if is_prime(1+b+c) {
                let primes = calculate_num_primes(b, c);
                if most_primes < primes {
                    most_primes = primes;
                    highest_b = b;
                    highest_c = c;
                }
            }
            c = next_prime(c);
        }
        c = 1;
    }
    println!("Highest b: {highest_b} Highest c: {highest_c}");
    println!("Result: {}", highest_b * highest_c);
    let elapsed = now.elapsed();
    println!("Elapsed {:.2?}", elapsed);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_primes() {
        let c = calculate_num_primes(1, 41);
        let e = 40;
        assert_eq!(c, e);
    }

    #[test]
    fn test_next_prime() {
        let c = next_prime(13);
        let e = 17;
        assert_eq!(c, e);
    }
}
