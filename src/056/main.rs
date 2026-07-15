use num_bigint::BigUint;
use std::time::Instant;

fn digital_sum(a: BigUint) -> BigUint {
    let mut t = a;
    let mut sum = BigUint::ZERO;
    let ten = BigUint::from(10 as u32);
    while t.clone() >= ten.clone() {
        sum += &t % &ten;
        t /= &ten;
    }
    sum + t
}

fn main() {
    let now = Instant::now();
    let mut largest = BigUint::ZERO;
    for i in 1..100 {
        for q in 1..100 {
            let j = BigUint::from(i as u32);
            let e = j.pow(q);
            let sum = digital_sum(e.clone());
            if sum > largest {
                largest = sum;
            }
        }
    }
    println!("Maximum digital sum {largest}");
    let elapsed = now.elapsed();
    println!("Took {} mils", elapsed.as_millis());
}
