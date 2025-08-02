use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::cast::ToPrimitive;

fn factorial(base: u32) -> BigUint {
    let mut res: BigUint = BigUint::from(1 as u32);

    for i in 1..base {
        res = res * i;
    }
    res
}

fn sum_digits(num: BigUint) -> u64 {
    let mut res: u64 = 0;
    let mut v = num;
    while v > BigUint::from(0 as u32) {
        let (q, r) = v.div_rem(&(10 as u32).into());
        let k = r.to_u64();
        res = res + k.unwrap();
        v = q;
    }
    res

}

fn main() {
    let a = sum_digits(factorial(100));
    print!("{:?}\n", a);
}
