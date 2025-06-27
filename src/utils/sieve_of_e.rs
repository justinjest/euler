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

pub fn get_nth_prime (nth: usize) -> usize {
    let mut total_primes = 0;
    let mut size_factor = 2;
    let mut primes = Vec::new();
    // Our sieve function includes 0 and 1, so we need to remove those
    while total_primes < nth + 2 {
        let mut s = nth * size_factor;
        primes = vec_primes(s);
        total_primes = primes.len();
        size_factor += 1;
    }
    // We don't expect our users to input 0 aligned items
    return primes[nth + 1];
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
