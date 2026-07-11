use std::time::Instant;

// create boolean list of len digits
// set vec[0] to true
// process number with vec in this location
// if vec[n + 1] = true
// if vec[n + 2] < vec.len
// exit
// vec[n + 2] = true
// vec[n] = false
// vec[0] = true

fn count_digits(x: usize) -> Vec<usize> {
    let mut output = vec![0; 10];
    let mut t = x;
    while t > 9 {
        let digit = t % 10;
        output[digit] += 1;
        t = t / 10;
    }
    output[t] += 1;
    output
}

fn digits_match(x: &Vec<usize>, y: &Vec<usize>) -> bool {
    if x.len() != y.len() {
        return false;
    }

    for i in 0..x.len() {
        if x[i] != y[i] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_match_01() {
        let a = vec![0; 10];
        let b = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let c = digits_match(&a, &b);
        let e = true;
        assert_eq!(c, e);
    }

    #[test]
    fn digits_match_02() {
        let a = count_digits(9876543210);
        let b = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let c = digits_match(&a, &b);
        let e = true;
        assert_eq!(c, e);
    }

    #[test]
    fn digits_match_03() {
        let a = count_digits(125874);
        let b = count_digits(251748);
        let c = digits_match(&a, &b);
        let e = true;
        assert_eq!(c, e);
    }
}

fn main() {
    let now = Instant::now();
    let mut test = 1;
    loop {
        let a = &count_digits(test);
        let b = &count_digits(test * 2);
        let c = &count_digits(test * 3);
        let d = &count_digits(test * 4);
        let e = &count_digits(test * 5);
        let f = &count_digits(test * 6);
        if digits_match(a, b)
            && digits_match(a, c)
            && digits_match(a, d)
            && digits_match(a, e)
            && digits_match(a, f)
        {
            println!("Found answer! {}", test);
            break;
        }
        test += 1;
    }
    let elapsed = now.elapsed();
    println!("Took {} mils", elapsed.as_millis());
}
