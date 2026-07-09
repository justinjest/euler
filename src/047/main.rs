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

    fn increase_size(&mut self, size: usize) {
        let array = &mut self.primes;
        let mut add = vec![true; size * 2];
        array.append(&mut add);

        self.primes = PrimeArray::sieve(array.to_vec());
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

    fn prime_factor(&mut self, num: usize) -> Vec<usize> {
        if self.primes.len() <= num {
            self.increase_size(num);
        }
        let mut vals = vec![];
        for i in 2..self.primes.len() {
            if num == 1 {
                return vals;
            } else if self.primes[i] == true {
                if num % i == 0 {
                    let n = num / i;
                    vals.push(i);
                    vals.append(&mut self.prime_factor(n));
                    vals.sort_unstable();
                    vals.dedup();
                    return vals;
                }
            }
        }
        vals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_factor_01() {
        let c = PrimeArray::new(100).prime_factor(14);
        let e = vec![2, 7];
        assert_eq!(c, e);
    }

    #[test]
    fn prime_factor_02() {
        let c = PrimeArray::new(10).prime_factor(15);
        let e = vec![3, 5];
        assert_eq!(c, e);
    }

    #[test]
    fn prime_factor_03() {
        let c = PrimeArray::new(10).prime_factor(17);
        let e = vec![17];
        assert_eq!(c, e);
    }

    #[test]
    fn prime_factor_04() {
        let c = PrimeArray::new(10).prime_factor(9);
        let e = vec![3];
        assert_eq!(c, e);
    }
}

fn main() {
    let now = Instant::now();
    let mut p = PrimeArray::new(100);
    let mut t1 = vec![];
    let mut t2 = vec![];
    let mut t3 = vec![];
    let mut t4 = vec![];
    let mut n = 5;
    loop {
        println!("testion {}", n);
        t4 = t3;
        t3 = t2;
        t2 = t1;
        t1 = p.prime_factor(n);
        if t1.len() == t2.len() && t1.len() == t3.len() && t1.len() == t4.len() && t1.len() == 4 {
            println!("Found the answer! {}, {}, {}, {}", n, n - 1, n - 2, n - 3);
            break;
        }
        n += 1;
    }
    let elapsed = now.elapsed();
    println!("Took {} seconds", elapsed.as_secs());
}
