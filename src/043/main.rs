use core::option::Option::None;
use std::time::Instant;

fn is_subdividable(num: usize) -> bool {
    let text = num.to_string();
    let array = [2, 3, 5, 7, 11, 13, 17];
    for i in 1..=7 {
        let mut iter = text.char_indices();
        let (start, _) = iter.nth(i).unwrap();
        let (end, _) = iter.nth(1).unwrap();
        let slice: usize = text[start..=end].parse().unwrap();
        if slice % array[i - 1] != 0 {
            return false;
        }
    }
    true
}

fn generate_pandigital() -> Vec<usize> {
    let mut digits = vec![1, 0, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut res = vec![];
    while next(&digits) != None {
        digits = next(&digits).unwrap();
        res.push(
            digits
                .iter()
                .map(|x| x.to_string())
                .collect::<String>()
                .parse()
                .unwrap(),
        );
    }
    res
}

fn next(vector: &Vec<usize>) -> Option<Vec<usize>> {
    let mut input = vector.to_vec();
    for k in (0..input.len() - 1).rev() {
        if input[k] < input[k + 1] {
            for l in (0..input.len()).rev() {
                if input[k] < input[l] {
                    let tmp = input[k];
                    input[k] = input[l];
                    input[l] = tmp;
                    let iter = &input.clone()[k..];
                    for i in 0..iter.len() - 1 {
                        input[k + 1 + i] = iter[iter.len() - 1 - i];
                    }
                    return Some(input);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_01() {
        let check = next(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let expect = Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 9, 8]);
        assert_eq!(check, expect);
    }

    #[test]
    fn next_02() {
        let check = next(&vec![1, 2, 3, 4]);
        let expect = Some(vec![1, 2, 4, 3]);
        assert_eq!(check, expect);
    }

    #[test]
    fn next_03() {
        let check = next(&vec![1, 2, 4, 3]);
        let expect = Some(vec![1, 3, 2, 4]);
        assert_eq!(check, expect);
    }

    #[test]
    fn is_subdividable_01() {
        let check = is_subdividable(1406357289);
        let expect = true;
        assert_eq!(check, expect);
    }

    #[test]
    fn is_subdividable_02() {
        let check = is_subdividable(1406357298);
        let expect = false;
        assert_eq!(check, expect);
    }
}

fn main() {
    let now = Instant::now();
    let mut count = 0;
    let mut sum = 0;
    let pandigital = generate_pandigital();
    for num in pandigital {
        if is_subdividable(num) {
            count += 1;
            sum += num;
        }
    }
    println!("Total counted digits {}", count);
    println!("Total sum is {}", sum);
    let elapsed = now.elapsed();
    println!("Took {} seconds", elapsed.as_secs());
}
