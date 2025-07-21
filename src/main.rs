#![allow(dead_code)]

mod utils;
mod highly_divisable_numbers;

fn main() {
    let res = highly_divisable_numbers::find_highly_divisable_triangle_number(500);
    print!("result: {res}\n");
}
