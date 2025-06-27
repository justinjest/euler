#![allow(dead_code)]

mod utils;
mod sum_square_difference;
fn main() {
    let ans = utils::sieve_of_e::get_nth_prime(10_001);
    println!("{:?}", ans);
}
