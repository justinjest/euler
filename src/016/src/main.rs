use num_bigint::BigInt;
use num_integer::Integer;

fn power_digit_sum(num: u32, power: u32) -> BigInt{
    let mut val: BigInt = BigInt::from(num).pow(power);
    let mut res: BigInt = BigInt::from(0);
    let mut escape = 0;
    while val > BigInt::from(0) || escape < 5 {
        let (q, r) = val.div_rem(&10.into());
        print!("{:?}\n", val);
        res = res + r;
        val = q;
        escape += 1;
    }
    return res.into()
}


fn main() {
    assert_eq!(power_digit_sum(2, 15), BigInt::from(26));
    let v = power_digit_sum(2, 1000);
    print!("{v}\n");
}
