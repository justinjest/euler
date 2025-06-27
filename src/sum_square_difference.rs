
fn sum_of_squares(num: usize) -> usize {
    let mut res = 0;
    for i in 1..num+1 {
        res += i*i;
    }
    return res;
}


fn square_of_sum(num: usize) -> usize {
    let res: usize;
    let mut tmp = 0;
    for i in 1..num+1 {
        tmp += i;
    }
    res = tmp*tmp;
    return res;
}

pub fn difference(num:usize) -> usize {
    square_of_sum(num) - sum_of_squares(num)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_of_squares() {
        let res = sum_of_squares(10);
        assert_eq!(res, 385);
    }
    #[test]
    fn test_square_of_sum() {
        let res = square_of_sum(10);
        assert_eq!(res, 3025);
    }
    #[test]
    fn test_difference() {
        let res = difference(10);
        assert_eq!(res, 2640);
    }
}
