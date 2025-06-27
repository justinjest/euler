
mod fib;
use fib::*;

fn main() {
    let ans = fib_sequence_max(4_000_000);
    println!("{:?}", ans);
    let mut val = 0;
    for i in ans {
        if i % 2 == 0 {
            val += i
        }
    }
    println!("{:?}", val);
}
