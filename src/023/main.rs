
fn find_factors(num: i32) -> Vec<i32> {
    let mut res = Vec::from([1,]);
    for i in 2..=num.isqrt(){
        if num % i == 0 {
            res.push(i);
            if num/i != i {
                res.push(num/i);
            }
        }
    }
    res
}

fn is_abundent(num:i32) -> bool {
    let abundent: i32 = find_factors(num).into_iter().sum();
    if abundent > num {
        return true;
    }
    false
}

fn main() {
    let abundent_list: Vec<i32> = (1..=28123)
        .filter(|n| is_abundent(*n))
        .collect();
    println!("{:?}", abundent_list);

    let mut can_be_written = vec![false; 28123+1];

    for (i, &a) in abundent_list.iter().enumerate() {
        for &b in &abundent_list[i..] {
            let sum = a + b;
            if sum <= 28123 {
                can_be_written[sum as usize] = true;
            } else {
                break;
            }
        }
    }

    let result: u32 = can_be_written
        .iter()
        .enumerate()
        .filter_map(|(i, &can)| if !can {Some(i as u32)} else {None})
        .sum();

    println!("{result}");
}
