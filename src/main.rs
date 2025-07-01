#![allow(dead_code)]

mod utils;
mod special_pythagorean_tripple;
fn main() {
    let (a, b, c) = special_pythagorean_tripple::find_val(1000, 2);
    println!("{:?}", (a*b*c));
}
