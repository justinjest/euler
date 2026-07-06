fn raise_to_the_fifth(value: usize) -> usize {
    return value.pow(5);
}

fn factorial(value: usize) -> usize {
    // I'm cheating this because rust doesn't have any
    if value <= 1 {
        return 1;
    } else {
        return value * factorial(value - 1);
    }
}

fn decompose_number(value: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut num = value;
    while num > 9 {
        v.push(num % 10);
        num = num / 10;
    }
    v.push(num);
    return v;
}

fn sum_fifths(digits: Vec<usize>) -> usize {
    let mut res = 0;
    let iter = digits.iter();
    for val in iter {
        res += raise_to_the_fifth(*val);
    }
    return res;
}

fn sum_factorial(digits: Vec<usize>) -> usize {
    let mut res = 0;
    let iter = digits.iter();
    for val in iter {
        res += factorial(*val);
    }
    return res;
}

fn process_value_fifths(value: usize) -> usize {
    let v = decompose_number(value);
    let res = sum_fifths(v);
    return res;
}

fn process_value_factorial(value: usize) -> usize {
    let v = decompose_number(value);
    let res = sum_factorial(v);
    return res;
}

fn find_value_factorial() -> usize {
    let mut test = 10;
    let mut res = 0;

    while test < 1000000 {
        if test == process_value_factorial(test) {
            res += test;
        }
        println!("current val: {} current sum {}", test, res);
        test += 1;
    }
    return res;
}

fn find_value_fifths() -> usize {
    let mut test = 10;
    let mut res = 0;
    while test < 1000000 {
        if test == process_value_fifths(test) {
            res += test;
        }
        println!("current val: {} current sum: {}", test, res);
        test += 1;
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_factorial_01() {
        let res = process_value_factorial(145);
        let expect = 145;
        assert_eq!(res, expect);
    }

    #[test]
    fn factorial_01() {
        let res = factorial(9);
        let expect = 362880;
        assert_eq!(res, expect);
    }

    #[test]
    fn sum_fifths_test01() {
        let res = sum_fifths(vec![2]);
        let expect = 32;
        assert_eq!(res, expect);
    }

    #[test]
    fn sum_fifths_test02() {
        let res = sum_fifths(vec![2, 2]);
        let expect = 64;
        assert_eq!(res, expect);
    }
    #[test]
    fn decompse_number_test01() {
        let res = decompose_number(1);
        let v = vec![1];
        assert_eq!(res, v);
    }

    #[test]
    fn decome_number_test02() {
        let res = decompose_number(1634);
        let v = vec![4, 3, 6, 1];
        assert_eq!(res, v);
    }

    #[test]
    fn raises_to_the_fifth_test01() {
        let res = raise_to_the_fifth(1);
        assert_eq!(res, 1);
    }

    #[test]
    fn raises_to_the_fifth_test02() {
        let res = raise_to_the_fifth(2);
        assert_eq!(res, 32);
    }
}

fn main() {
    let min = find_value_factorial();
    println!("{}", min);
}
