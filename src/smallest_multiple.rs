pub fn smallest_multiple_func(num: usize) -> usize {
    let mut ans = num;
    let mut solved = false;
    while !solved {
        if divide_test(ans, num) {
            solved = true;
            return ans;
        }
        ans += 1;
    }
    return 0;
}

fn divide_test(num: usize, range: usize) -> bool {
    for v in 2..range+1{
        if num%v == 0 {
            if v == range {
                println!("Found answer {num}");
                return true;
            }
        } else {
            return false;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smallest_multiple() {
        let res = smallest_multiple_func(10);
        assert_eq!(res, 2520);
    }
}
