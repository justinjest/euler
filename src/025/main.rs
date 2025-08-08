use num_bigint::BigUint;
use num_traits::One;

fn main() {
    let mut num0 = BigUint::ZERO;
    let mut num1 = BigUint::one();
    let mut index = 0;
    let cap: BigUint = (10 as u32).into();
    let c = cap.pow(999);
    while num0 < c {
        let num2 = num0 + &num1;
        num0 = num1;
        num1 = num2;
        index += 1;
    }
    println!("{}", num0);
    println!("{}", index);
}
