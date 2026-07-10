use std::time::Instant;

struct PrimeArray {
    primes: Vec<bool>,
}

impl PrimeArray {
    fn new(size: usize) -> Self {
        if size < 2 {
            panic!("Unable to initalize a prime array less then size 2")
        }
        let mut array = vec![true; size + 1];

        array[0] = false;
        array[1] = false;

        Self {
            primes: PrimeArray::sieve(array),
        }
    }

    #[allow(unused)]
    fn print_primes(&self) {
        let size = self.primes.len();
        for x in 0..size {
            if self.primes[x] == true {
                println!("{} is prime!", x);
            }
        }
    }

    #[allow(unused)]
    fn is_prime(&self, num: usize) -> bool {
        self.primes[num]
    }

    // Takes and turns PrimeArray into a useful vec
    fn sieve(mut array: Vec<bool>) -> Vec<bool> {
        let size = array.len();
        for x in 3..=size.isqrt() {
            if array[x] == true {
                let mut val = x * x;
                while val < size {
                    array[val] = false;
                    val += x;
                }
            }
        }
        array
    }
}

fn count_digits(x: usize) -> Vec<usize> {
    let mut output = vec![0; 10];
    let mut t = x;
    while t > 9 {
        let digit = t % 10;
        output[digit] += 1;
        t = t / 10;
    }
    output[t] += 1;
    output
}

fn is_pandigital(number: usize) -> bool {
    let num_digits = number.checked_ilog10().unwrap() as usize;
    let digits = count_digits(number);
    if digits[0] != 0 {
        return false;
    }
    for i in &digits[1..=num_digits + 1] {
        if *i != 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_pandigital_01() {
        let check = is_pandigital(2143);
        let expect = true;
        assert_eq!(check, expect);
    }

    #[test]
    fn is_pandigital_02() {
        let check = is_pandigital(987654321);
        let expect = true;
        assert_eq!(check, expect);
    }

    #[test]
    fn is_pandigital_03() {
        let check = is_pandigital(1111);
        let expect = false;
        assert_eq!(check, expect);
    }

    #[test]
    fn is_pandigital_04() {
        let check = is_pandigital(99);
        let expect = false;
        assert_eq!(check, expect);
    }

    #[test]
    fn is_pandigital_05() {
        let check = is_pandigital(97654321);
        let expect = false;
        assert_eq!(check, expect);
    }
}

fn main() {
    let now = Instant::now();
    let primes = PrimeArray::new(100_000_000);
    let mut largest_prime = 0;
    for num in 0..primes.primes.len() {
        if primes.primes[num] && is_pandigital(num) {
            largest_prime = num;
        }
    }
    println!("Largest found prime is {}", largest_prime);
    let elapsed = now.elapsed();
    println!("Took {} miliseconds", elapsed.as_millis());
}
