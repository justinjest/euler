use crate::utils::sieve_of_e;

pub fn factorization(num: usize) -> Vec<usize> {
    println!("Starting factorization");
    let mut ans = Vec::new();
    for n in 1..num.isqrt() + 1{
        if num % n == 0 {
            println!("{:?}", n);
            ans.push(n);
        }
    }
    return ans;
}

pub fn largest_prime(num: usize) -> usize {
    let mut ans = factorization(num);
    loop {
        if ans.len() <=0 {
            break 1;
        }
        let val = ans.pop().unwrap();
        if sieve_of_e::is_prime(val) {
            return val;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let res = largest_prime(13195);
        assert_eq!(res, 29);
    }
}
