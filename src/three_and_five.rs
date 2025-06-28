pub fn three_and_five (num: usize) -> usize {
    let mut ans = 0;
    for n in 1..num {
        if n % 3 == 0 {
            ans += n;
        } else if n % 5 == 0 {
            ans += n;
        }
    }
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let res = three_and_five(10);
        assert_eq!(res, 23);
    }
}
