use std::collections::HashSet;

fn find_all_pandigital() -> Vec<usize> {
    let mut x = 1;
    let mut y = 1;
    let mut pandigital_sums = vec![];

    while x <= 65 {
        while y <= 987 {
            println! {"x:{}, y: {}", x, y};
            if product_is_pandigital(x, y) {
                pandigital_sums.push(x * y);
            }
            y += 1
        }
        x += 1;
        y = x;
    }
    x = 1234; // Smallest four digit pandigital number
    while x <= 9876 {
        while y <= 9 {
            println! {"x:{}, y: {}", x, y};
            if product_is_pandigital(x, y) {
                pandigital_sums.push(x * y);
            }
            y += 1
        }
        x += 1;
        y = 1;
    }
    pandigital_sums
}

fn sum_unique_items_in_vec(items: Vec<usize>) -> usize {
    let res = items.iter().copied().collect::<HashSet<_>>().iter().sum();
    res
}

fn product_is_pandigital(x: usize, y: usize) -> bool {
    is_pandigital(x, y, x * y)
}

// Returns true when x, y, and z contains each number exactly once
// While we do count the number of zeros we only care about 1-9
// hence digit starts at 1 and ends at 9
fn is_pandigital(x: usize, y: usize, z: usize) -> bool {
    let x_vec = count_digits(x);
    let y_vec = count_digits(y);
    let z_vec = count_digits(z);
    // if they end up being different sizes panic
    assert_eq!(x_vec.len(), 10);
    assert_eq!(y_vec.len(), 10);
    assert_eq!(z_vec.len(), 10);
    let mut digit = 0;
    while digit <= 9 {
        let tmp = x_vec[digit] + y_vec[digit] + z_vec[digit];
        // If there are zeros we should bail
        if digit == 0 && tmp != 0 {
            return false;
        }
        if tmp != 1 && digit > 0 {
            return false;
        }
        digit += 1;
    }
    true
}

// This will output a vec with size 10, since 0 is a countable digit
// Each location in this array will provide the number of times the digit was
// in the number
fn count_digits(x: usize) -> Vec<usize> {
    let mut output = vec![0; 10];
    let mut t = x;
    while t > 9 {
        let digit = t % 10;
        output[digit] += 1;
        t = t / 10;
    }
    output[t] += 1;
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_is_pandigital_01() {
        let res = product_is_pandigital(39, 186);
        let expect = true;
        assert_eq!(res, expect);
    }

    #[test]
    fn pandigital_test_01() {
        let res = is_pandigital(39, 186, 7524);
        let expect = true;
        assert_eq!(res, expect);
    }

    #[test]
    fn pandigital_test_02() {
        let res = is_pandigital(1, 2, 1);
        let expect = false;
        assert_eq!(res, expect);
    }

    #[test]
    fn pandigital_test_03() {
        let res = is_pandigital(9876, 543, 210);
        let expect = false;
        assert_eq!(res, expect);
    }

    #[test]
    fn count_digits_01() {
        let res = count_digits(39);
        let expect = [0, 0, 0, 1, 0, 0, 0, 0, 0, 1];
        assert_eq!(res, expect);
    }
}

fn main() {
    println!("Hello, world!");
    let pandigitals = find_all_pandigital();
    let product = sum_unique_items_in_vec(pandigitals);
    println!("result: {}", product);
}
