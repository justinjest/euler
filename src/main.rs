#![allow(dead_code)]

mod utils;
mod summation_of_primes;
fn main() {
    let res = summation_of_primes::sum_of_primes(2_000_000);
    println!("{:?}", res);
}
