mod utils;
mod factor;

fn main() {
    let ans = factor::largest_prime(600851475143);
    println!("{:?}", ans);
}
