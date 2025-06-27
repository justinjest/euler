#![allow(dead_code)]

mod utils;
mod smallest_multiple;
fn main() {
    let ans = smallest_multiple::smallest_multiple_func(20);
    println!("{:?}", ans);
}
