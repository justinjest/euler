#![allow(dead_code)]

mod utils;
mod lattice_problem;

fn main() {
    let res = lattice_problem::lattice_path(20, 20);
    print!("result: {res}\n");
}
