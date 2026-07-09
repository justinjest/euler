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
    fn is_prime(&mut self, number: usize) -> bool {
        if number >= self.primes.len() {
            self.increase_size(number * 2);
        }

        self.primes[number]
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

    #[allow(unused)]
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

fn get_permutation(number: usize) -> Vec<usize> {
    let mut n = number;
    let mut digits: Vec<usize> = vec![];
    while n > 9 {
        let last = n % 10;
        digits.push(last);
        n = n / 10;
    }
    digits.push(n);
    digits.sort();
    let mut res = vec![];
    while next(&digits) != None {
        digits = next(&digits).unwrap();
        res.push(digits.iter().fold(0usize, |acc, &d| acc * 10 + d));
    }
    res
}

fn next(vector: &Vec<usize>) -> Option<Vec<usize>> {
    let mut input = vector.to_vec();
    for k in (0..input.len() - 1).rev() {
        if input[k] < input[k + 1] {
            for l in (0..input.len()).rev() {
                if input[k] < input[l] {
                    let tmp = input[k];
                    input[k] = input[l];
                    input[l] = tmp;
                    let iter = &input.clone()[k..];
                    for i in 0..iter.len() - 1 {
                        input[k + 1 + i] = iter[iter.len() - 1 - i];
                    }
                    return Some(input);
                }
            }
        }
    }
    None
}

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    fn test_prime_sieve_01() {
        let a = PrimeArray::new(10000);
        let c = a.primes[8714];
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
}

fn main() {
    let now = Instant::now();
    let mut primes = PrimeArray::new(10_000);
    for i in 1_000..=9_999 {
        let perms = get_permutation(i);
        for j in i..10_000 - i {
            if i + 2 * j >= 10_000 {
                break;
            }
            if primes.is_prime(i)
                && primes.is_prime(i + j)
                && primes.is_prime(i + 2 * j)
                && perms.contains(&(i + j))
                && perms.contains(&(i + 2 * j))
            {
                println!("Found tripplet {}{}{}", i, i + j, i + 2 * j);
            }
        }
    }
    let elapsed = now.elapsed();
    println!("Took {} mils", elapsed.as_millis());
}
