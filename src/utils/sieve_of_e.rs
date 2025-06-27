#[ignore]
pub fn sieve(max: usize) -> Vec<bool> {
    println!("Starting sieve");
    let mut res = vec![true; max+1];
    for n in 2..max.isqrt() + 1{
        if res[n] {
            let mut filter = n*n;
            while filter <= max {
                res[filter] = false;
                filter += n;
            }
        }
    }
    return res;
}

pub fn is_prime(num: usize) -> bool {
    for n in 2..num.isqrt() + 1 {
        if num%n == 0 {
            return false;
        }
    }
    return true;
}

pub fn vec_primes(max: usize) -> Vec<usize> {
    let mut res = Vec::new();
    let mut index = 0;
    let vec = sieve(max);
    for val in vec {
        if val {
            res.push(index)
        }
        index += 1;
    }
    return res;
}
