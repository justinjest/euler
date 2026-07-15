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

struct Sides {
    s1: usize,
    s2: usize,
    s3: usize,
    s4: usize,
}

impl Sides {
    pub fn to_vec(&self) -> Vec<usize> {
        vec![self.s1, self.s2, self.s3, self.s4]
    }
}

fn get_corners(side_length: usize) -> Option<Sides> {
    if side_length % 2 == 0 {
        return None;
    }
    let delta = side_length - 1;
    let s1 = side_length.pow(2);
    let s2 = s1 - delta;
    let s3 = s2 - delta;
    let s4 = s3 - delta;

    Some(Sides { s1, s2, s3, s4 })
}

fn main() {
    let now = Instant::now();
    let mut sieve = PrimeArray::new(1_000_000_000);
    let mut primes = 0;
    let mut total = 1;
    let mut edge_length = 3;
    loop {
        println!(
            "Edge length is currently {edge_length} ratio is {}",
            (primes as f64 / total as f64)
        );
        let corners = get_corners(edge_length).expect("Unable to unwrap");
        for corner in corners.to_vec() {
            if sieve.is_prime(corner) {
                primes += 1;
            }
            total = edge_length * 2 - 1;
        }
        if (primes as f64 / total as f64) < 0.10 {
            println!("Prime ratio dropped too low at {edge_length}");
            break;
        }
        edge_length += 2;
    }
    println!("{primes} found out of a total of {total}");
    let elapsed = now.elapsed();
    println!("Took {} mils", elapsed.as_millis());
}
