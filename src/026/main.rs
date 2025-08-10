use std::time::Instant;
use std::collections::HashMap;

fn get_len_repeats(n: u32) -> u32 {
    let mut rem = 1;
    let mut digits = HashMap::new();
    let mut i: u32 = 1;

    loop {
        rem = (10 * rem) % n;
        if digits.contains_key(&rem){
            return i - digits[&rem]
        }
        digits.insert(rem, i);
        i += 1;
    }
}

fn find_next_prime(n: u32) -> u32 {
    let next_num = n + 1;
    for i in 2..next_num.isqrt() {
        if next_num % i == 0 {
            return find_next_prime(next_num);
        }
    }
    next_num
}

fn main() {
    let now = Instant::now();
    let mut largest = 0;
    let mut index = 0;
    let mut i = 1;
    while i < 1000 {
        let len = get_len_repeats(i);
        if len > largest {
            largest = len;
            index = i;
        }
        i = find_next_prime(i);
    }
    println!("{index}:len {largest}");
    let elapsed = now.elapsed();
    println!("Elapsed {:.2?}", elapsed);
}
