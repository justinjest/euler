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
            primes: sieve(array),
        }
    }

    fn print_primes(&self) {
        let size = self.primes.len();
        for x in 0..size {
            if self.primes[x] == true {
                println!("{} is prime!", x);
            }
        }
    }

    fn is_prime(&self, num: usize) -> bool {
        self.primes[num]
    }

    fn rotated_primes(&self) -> Vec<usize> {
        let mut ans = vec![];
        let mut pass = true;
        for x in 0..self.primes.len() {
            if self.primes[x] == true {
                let mut test = x;
                let num_digits = test.checked_ilog10().unwrap();
                for _ in 0..num_digits {
                    let last_digit = test % 10;
                    test = (test / 10) + last_digit * 10_usize.pow(num_digits);
                    if !self.is_prime(test) {
                        pass = false;
                    }
                }
                if pass {
                    ans.push(x);
                }
                pass = true;
            }
        }
        ans
    }
}

// Takes and turns PrimeArray into a useful vec
fn sieve(mut array: Vec<bool>) -> Vec<bool> {
    let size = array.len();
    for x in 0..size {
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

fn main() {
    let now = Instant::now();
    let primes = PrimeArray::new(1_000_000);
    let elapsed_time = now.elapsed();
    println!(
        "Calculating primes took {} seconds and {} miliseconds",
        elapsed_time.as_secs(),
        elapsed_time.subsec_millis()
    );
    primes.print_primes();
    let res = primes.rotated_primes();
    for i in res.clone() {
        println!("{} is a rotated prime", i);
    }
    println!("There are {} rotated primes", res.len());
}
