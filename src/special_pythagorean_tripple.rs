
pub fn create_tripple(m: usize, n: usize) -> (usize,usize,usize) {
    let a = m*m - n*n;
    let b = 2*m*n;
    let c = m*m + n*n;
    return (a, b, c);
}

pub fn find_val(val: usize,m: usize) -> (usize,usize,usize) {
    if m < 2 {
        println!("Value less than 2");
        return (0, 0, 0);
    }
    for t in 1..m {
        let (a,b,c) = create_tripple(m, t);
        let sum = a + b + c;
        println!("{sum}");
        if sum == val {
            return (a, b, c);
        }
// There needs to be some offramp, and I'm assuming by the time you get to val^2
// You either will have found a valid tripple, or none exist
// This may be an invalid assumption
        else if sum > val*val {
            println!("{m}, {t}");
            println!("{:?}", (a, b, c));
            return (0, 0, 0);
        }
    }
    find_val(val, m+1)
}


#[cfg(test)]
mod tests {
    use super::*;
    fn test_find_val() {
        let res = find_val(12, 2);
        assert_eq!(res, (3, 4, 5));
        let res = find_val(234, 2);
        assert_eq!(res, (65, 72, 97));
    }
}
