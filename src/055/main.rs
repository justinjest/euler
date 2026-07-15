use std::time::Instant;

fn is_palindrome(num: u128) -> bool {
    let forward = num.to_string();
    let backward = forward.clone().chars().rev().collect::<String>();

    forward == backward
}

fn flip_and_add(num: u128) -> u128 {
    let mut s: String = num.to_string();
    s = s.chars().rev().collect::<String>();
    let n: u128 = s.parse().expect("Unable to reverse");
    num + n
}

fn is_lychrel(num: u128) -> bool {
    let mut n = num;
    for _ in 1..=50 {
        n = flip_and_add(n);
        if is_palindrome(n) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_lychrel_01() {
        let c = is_lychrel(47);
        let e = false;
        assert_eq!(c, e);
    }

    #[test]
    fn is_lychrel_02() {
        let c = is_lychrel(196);
        let e = true;
        assert_eq!(c, e);
    }

    #[test]
    fn is_lychrel_03() {
        let c = is_lychrel(4994);
        let e = true;
        assert_eq!(c, e);
    }

    #[test]
    fn is_lychrel_04() {
        let c = is_lychrel(349);
        let e = false;
        assert_eq!(c, e);
    }

    #[test]
    fn max_lychrel() {
        let c = is_lychrel(9999);
        assert!(c);
    }
}

fn main() {
    let now = Instant::now();
    let mut count = 0;
    for i in 1..10000 {
        if is_lychrel(i) {
            println!("{} is lychrel", i);
            count += 1;
        }
    }
    println!("Total count is {}", count);
    let elapsed = now.elapsed();
    println!("Took {} mils", elapsed.as_millis());
}
