#![allow(dead_code)]

mod utils;
mod collatz;

fn main() {
    let res = collatz::largest(1_000_000);
    print!("result: {res}\n");
}
