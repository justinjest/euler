use std::collections::HashMap;


fn count_letters (num: u32) -> u32 {
    match num {
        1..=1000 => (),
        _ => panic!("Number outside expected range")
    }
    let c = HashMap::from([
        (0, 0),  (1, 3),  (2, 3),  (3, 5),   (4, 4),
        (5, 4),  (6, 3),  (7, 5),  (8, 5),   (9, 4),
        (10, 3), (11, 6), (12, 6), (13, 8),  (14, 8),
        (15, 7), (16, 7), (17, 9), (18, 8),  (19, 8),
        (20, 6), (30, 6), (40, 5), (50, 5),  (60, 5),
        (70, 7), (80, 6), (90, 6), (100, 7), (1000, 8),
    ]);
    let mut sum = 0;
    if num == 1000 {
        return c[&1] + c[&1000]; // Hard coded cap one thousand
    }

    let hundreds = num / 100;
    let tens = num % 100;
    let ones = num % 10;
    print!("{hundreds}, {tens}, {ones}\n");
    if hundreds >= 1 {
        sum += c[&hundreds];
        sum += c[&100];
    }

    match tens {
        (10..20) => sum += c[&tens],
        0 => (),
        _ => sum += c[&(tens / 10 * 10)]
    }

    if ones >= 1 && !(10..20).contains(&tens) {
        sum += c[&ones];
    }

    if hundreds >= 1 && tens >= 1 {
        return sum + 3;
    }

    sum
}

fn main() {
    let c: u32 = (1..=1000).map(|x| count_letters(x)).sum();
    print!("{c}\n");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_letter() {
        let c = count_letters(1);
        assert_eq!(c, 3);
    }

    #[test]
    fn test_compound_letter() {
        let c = count_letters(370);
        assert_eq!(c, 22);
    }

    #[test]
    fn test_given_one() {
        let c = count_letters(342);
        assert_eq!(c, 23);
    }

    #[test]
    fn test_given_two() {
        let c = count_letters(115);
        assert_eq!(c, 20);
    }

    #[test]
    fn test_iterator() {
        let c: u32 = (1..=5).map(|x| count_letters(x)).sum();
        assert_eq!(c, 19);
    }
}
