pub fn sieve(max: usize) -> Vec<bool> {
    let mut res = vec![true; max+1];
    for n in 2..max.isqrt() + 1{
        if res[n] {
            let mut filter = n*n;
            while filter <= max {
                res[filter] = false;
                filter += n;
            }
        }
    }
    return res;
}

pub fn print_index(vec: &Vec<bool>) {
    let mut index = 0;
    for val in vec {
        if *val {
            println!("{index}");
        }
        index += 1;
    }
}
