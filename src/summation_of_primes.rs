use crate::utils;
//// This function will return the sum of all primes below max
//// If the max is a prime that will be included in the function
pub fn sum_of_primes(max: usize) -> usize {
    let mut sum = 0;
    while sum <= max {
    let primes = utils::sieve_of_e::vec_primes(max);
    let prime_iter = primes.iter();
    for val in prime_iter {
        sum += val;
    }
    }
    // our sieve function counts 1 as prime, which we must remove to comply with
    // project euler assumptions
    return sum-1;
}
