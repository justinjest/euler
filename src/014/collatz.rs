
// Returns the len of a collatz sequence for a given input
pub fn sequence(input:u64) -> u64 {
    let mut val = input;
    let mut seq = 1;
    while val != 1{
        if val % 2 == 0 {
            val = even(val)
        } else {
            val = odd(val)
        }
        seq += 1;
    }
    seq
}

pub fn largest(limit: u64) -> u64 {
    let mut largest = 0;
    for i in 1..limit {
        let val = sequence(i);
        if val > largest {
            print!("New largest! {i} went {val} times\n");
            largest = val;
        }
    }
    largest
}

// Returns even case for collatz n -> n/2
fn even(input: u64) -> u64 {
   input / 2
}


// Returns odd case for collatz n -> 3n + 1
fn odd(input: u64) -> u64 {
   3 * input + 1
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collatz() {
        let res = sequence(13);
        assert_eq!(res, 10);
    }
}
