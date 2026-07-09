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

fn find_incorrect_example() {
    let primes = PrimeArray::new(1_000_000);
    let mut n = 9;
    loop {
        println!("Testing {}", n);
        for x in 1..((n as usize).isqrt()) {
            if n <= 2 * x.pow(2) {
                println!("Doesn't work {}", n);
                return;
            }
            if primes.primes[n] || primes.primes[n - (2 * x.pow(2))] == true {
                n += 2;
                break;
            }
        }
    }
}

fn main() {
    let now = Instant::now();
    find_incorrect_example();
    let elapsed = now.elapsed();
    println!("Took {} seconds", elapsed.as_secs());
}
