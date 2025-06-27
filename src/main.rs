
mod sieve_of_e;
use sieve_of_e::*;

fn main() {
    let val = sieve(10);
    print_index(&val);
}
