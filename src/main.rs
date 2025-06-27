#![allow(dead_code)]

mod utils;
mod palindrome;
fn main() {
    let ans = palindrome::largest_palindrome(999, 999);
    println!("{:?}", ans);
}
