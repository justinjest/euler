#![allow(dead_code)]

mod utils;
mod sum_square_difference;
fn main() {
    let ans = sum_square_difference::difference(100);
    println!("{:?}", ans);
}
