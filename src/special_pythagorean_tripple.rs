
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
        println!("{:?}", (a, b, c));
        let sum = a + b + c;
        if sum == val {
            return (a, b, c);
        }else if sum > val {
            println!("sum overwent val {sum}");
            return (0, 0, 0);
        }
    }
    find_val(val, m+1)
}
