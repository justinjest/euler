fn raise_to_the_fifth(value: i32) -> i32 {
    return value.pow(5);
}

fn raise_to_the_fourth(value: i32) -> i32 {
    return value.pow(4);
}

fn decompose_number(value: i32) -> Vec<i32> {
    let mut v = Vec::new();
    let mut num = value;
    while num > 9 {
        v.push(num % 10);
        num = num / 10;
    }
    v.push(num);
    return v;
}

fn sum_fifths(digits: Vec<i32>) -> i32 {
    let mut res = 0;
    let iter = digits.iter();
    for val in iter {
        res += raise_to_the_fifth(*val);
    }
    return res;
}

fn sum_fourths(digits: Vec<i32>) -> i32 {
    let mut res = 0;
    let iter = digits.iter();
    for val in iter {
        res += raise_to_the_fourth(*val);
    }
    return res;
}

fn process_value_fifths(value: i32) -> i32 {
    let v = decompose_number(value);
    let res = sum_fifths(v);
    return res;
}

fn process_value_fourths(value: i32) -> i32 {
    let v = decompose_number(value);
    let res = sum_fourths(v);
    return res;
}

fn values_equal(v1: i32, v2: i32) -> bool {
    if v1 == v2 {
        return true;
    }
    return false;
}

fn find_value_fourths() -> i32 {
    let mut test = 10;
    let mut res = 0;
    while test < 10000 {
        if values_equal(test, process_value_fourths(test)) {
            res += test;
        }
        println!("{}", test);
        test += 1;
    }
    return res;
}

fn find_value_fifths() -> i32 {
    let mut test = 10;
    let mut res = 0;
    while test < 1000000 {
        if values_equal(test, process_value_fifths(test)) {
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
    fn fourth_test() {
        let val = find_value_fourths();
        let res = 19316;
        assert_eq!(val, res);
    }

    #[test]
    fn process_fourths_test01() {
        let val = process_value_fourths(1634);
        let res = 1634;
        assert_eq!(val, res);
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
    let min = find_value_fifths();
    println!("{}", min);
}
