
pub fn is_palindrome(num: usize) -> bool {
    let tmp = num.to_string();
    let tmp_vec: Vec<char> = tmp.chars().collect();
    for val in 0..tmp_vec.len() {
        if tmp_vec[val] != tmp_vec[tmp_vec.len() - 1 - val] {
            return false;
        }
    }
    return true;
}
//// Produces the largest palindrome number within the range of the two
//// provided numbers.
pub fn largest_palindrome(num0: usize, num1: usize) -> usize {
    // This is an extremely slow solution, but for smaller numbers it will work
    let mut max = 0;
    // We add one to both of these to get up to the number provided
    for i in 0..num0 + 1{
        for j in 0..num1 + 1{
            if is_palindrome(i*j) {
                if max < i*j {
                    max = i*j;
                }
            }
        }
    }
    return max
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_palindrome() {
        let res = is_palindrome(101);
        assert_eq!(res, true);
    }
    #[test]
    fn test_largest_palindrome() {
        let res = largest_palindrome(99, 99);
        assert_eq!(res, 9009);
    }
}
