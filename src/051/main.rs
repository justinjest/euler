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

// output is (digits in bitmask, number of found primes)
fn determine_primes(number: usize, bitmask: u16, primes: &mut PrimeArray) -> usize {
    let mut found_primes = 0;
    let num_digits = number.ilog10() + 1;
    let saftey_mask = (1 << num_digits) - 1;
    let b = bitmask & saftey_mask;

    if b == 0 {
        return 0;
    }

    let mut target_digit = None;
    for i in 0..num_digits {
        if (b & (1 << i)) != 0 {
            let place_value = 10_usize.pow(i);
            let digit = (number / place_value) % 10;
            if let Some(t) = target_digit {
                if digit != t {
                    return 0;
                }
            } else {
                target_digit = Some(digit);
            }
        }
    }

    for x in 0..10 {
        if x == 0 && (b & (1 << (num_digits - 1))) != 0 {
            continue;
        }

        let mut n = number;
        for i in 0..num_digits {
            let bit = 1 << i;

            if (b & bit) != 0 {
                let place_value = 10_usize.pow(i);
                let current_digit = (n / place_value) % 10;
                n = n - (current_digit * place_value) + (x * place_value);
            }
        }
        if primes.is_prime(n) {
            found_primes += 1;
        }
    }
    found_primes
}

fn main() {
    let now = Instant::now();
    let mut test: usize = 3;
    let mut primes = PrimeArray::new(10_000_000);
    'outer: loop {
        let num_digits = test.ilog10() + 1;
        let max_mask = 1 << num_digits;
        for i in 0..max_mask {
            let c = determine_primes(test, i, &mut primes);
            if c == 8 {
                println!("Found result {}  ", test);
                break 'outer;
            }
        }
        test += 2;
        while !primes.is_prime(test) {
            test += 2;
        }
        if test.ilog10() > u16::MAX.into() {
            panic!("Need to increase u16 to u32");
        }
    }
    let elapsed = now.elapsed();
    println!("Took {} mils", elapsed.as_millis());
}
