use std::time::Instant;

struct PrimeArray {
    primes_bool: Vec<bool>,
}

impl PrimeArray {
    fn new(size: usize) -> Self {
        if size < 2 {
            panic!("Unable to initalize a prime array less then size 2")
        }
        let mut array = vec![true; size + 1];

        array[0] = false;
        array[1] = false;

        let default = PrimeArray::sieve(array);
        Self {
            primes_bool: default,
        }
    }

    #[allow(unused)]
    fn is_prime(&mut self, number: usize) -> bool {
        if number >= self.primes_bool.len() {
            self.increase_size(number * 2);
        }

        self.primes_bool[number]
    }

    fn increase_size(&mut self, size: usize) {
        let array = &mut self.primes_bool;
        let mut add = vec![true; size * 2];
        array.append(&mut add);
        self.primes_bool = PrimeArray::sieve(array.to_vec());
    }

    // Takes and turns PrimeArray into a useful vec
    fn sieve(mut array: Vec<bool>) -> Vec<bool> {
        let size = array.len();
        for x in 2..=size.isqrt() {
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

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    fn test_prime_sieve_01() {
        let a = PrimeArray::new(10000);
        let c = a.primes_bool[8714];
        let e = false;
        assert_eq!(c, e);
    }

    #[test]
    fn test_prime_sieve_02() {
        let mut a = PrimeArray::new(10);
        let c = a.is_prime(2597);
        let e = false;
        assert_eq!(c, e);
    }

    #[test]
    fn test_prime_sieve_03() {
        let mut a = PrimeArray::new(20);
        let c = a.is_prime(953);
        let e = true;
        assert_eq!(c, e);
    }
}

fn main() {
    const TARGET: usize = 1_000_000;
    let now = Instant::now();
    let mut primes = PrimeArray::new(TARGET / 2);
    let mut prime_array = vec![];
    for i in 0..primes.primes_bool.len() {
        if primes.primes_bool[i] {
            prime_array.push(i);
        }
    }
    let mut largest_array = 0;

    for i in 0..prime_array.len() {
        for j in 0..i {
            let mut sum = 0;
            for x in j..=i {
                sum += prime_array[x];
            }
            if i > TARGET {
                break;
            }
            if primes.is_prime(sum) && sum < TARGET {
                if i - j + 1 > largest_array {
                    largest_array = i - j + 1;
                    println!(
                        "New largest array found {} uses {} primes",
                        sum, largest_array
                    );
                }
            }
        }
    }

    let elapsed = now.elapsed();
    println!("Took {} mils", elapsed.as_millis());
}
