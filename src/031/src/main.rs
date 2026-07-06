use std::time::Instant;
use std::collections::HashSet;
use num_bigint::{BigUint, ToBigUint};

fn main() {
    let now = Instant::now();
    let mut vals = HashSet::new();
    for a in 2..=100 {
        for b in 2..=100 {
            let a_val: BigUint = (a as u32).to_biguint().unwrap();
            let b_val: u32 = b;
            let res: BigUint = a_val.pow(b_val).into();
            vals.insert(res);
        }
    }
    println!("{:?}", vals.len());
    let elapsed = now.elapsed();
    println!("Elapsed {:.2?}", elapsed);
}
