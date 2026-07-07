fn is_palindrome(number: u64) -> bool {
    let string = number.to_string();
    let rev: String = string.chars().rev().collect();
    let bin = format!("{:b}", number);
    let revbin: String = bin.chars().rev().collect();
    string == rev && bin == revbin
}

fn main() {
    let mut sum = 0;
    for x in 1..1_000_000 {
        if is_palindrome(x) {
            println!("{} is palindromic", x);
            sum += x;
        }
    }
    println!("Total sum is {}", sum);
    println!("Hello world");
}
