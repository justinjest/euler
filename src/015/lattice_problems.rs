fn factorial (num: u128) -> u128 {
    print!("Factorial: {num}\n");
    let mut res = 1;
    for i in 2..num + 1{
        res = i * res;
    }
    res
}

fn binomial_coefficent(n: u32, k: u32) -> u128 {
    let mut res = 1 as u128;
    print!("{n}, {k}\n");
    if k > n {
        panic!("Unable to compute binomial coefficent when K > N {k} > {n}");
    }
    // k+1 will start the number at the last non cancelled number
    // It will then run until n
    for i in k+1..=n+k {
        res = u128::from(i) * res;
        print!("For num {i} the new val is {res}\n");
    }
    let num = factorial(u128::from(k));
    return res / num;
}

pub fn lattice_path(x: u32, y: u32) -> u128 {
    binomial_coefficent(x, y)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial_coefficent() {
        let r = binomial_coefficent(4, 2);
        let e = 12;
        assert_eq!(r, e);
    }

}
