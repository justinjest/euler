#[derive(Clone, Debug, PartialEq)]
struct Fraction {
    n: usize,
    d: usize,
}

fn find_all_examples() {
    let mut res = Fraction { n: 1, d: 1 };
    for a in 11..99 {
        for b in 11..100 {
            if test_simplifiy(Fraction { n: a, d: b }) {
                println!("Found fraction {}/{}", a, b);
                res = Fraction {
                    n: res.n * a,
                    d: res.d * b,
                };
            }
        }
    }
    let ans = simplify_fraction(res);
    println!("Result {}/{}", ans.n, ans.d);
}

// This tests to see if any of the four incorrectly reduced fractions equal
// the correct ammount
fn test_simplifiy(input: Fraction) -> bool {
    if input.n >= input.d {
        return false;
    }

    let n_ten = input.n / 10;
    let n_one = input.n % 10;
    let d_ten = input.d / 10;
    let d_one = input.d % 10;
    let simplify = simplify_fraction(input.clone());

    if n_one == 0 || d_one == 0 {
        return false;
    }

    if n_ten == d_ten {
        // frac_1
        let frac = Fraction { n: n_one, d: d_one };
        let test = simplify_fraction(frac);
        if simplify == test {
            return true;
        }
    }

    if n_ten == d_one {
        //frac_4
        let frac = Fraction { n: n_one, d: d_ten };
        let test = simplify_fraction(frac);
        if simplify == test {
            return true;
        }
    }

    if n_one == d_one {
        // frac_2
        let frac = Fraction { n: n_ten, d: d_ten };
        let test = simplify_fraction(frac);
        if simplify == test {
            return true;
        }
    }

    if n_one == d_ten {
        // frac_3
        let frac = Fraction { n: n_ten, d: d_one };
        let test = simplify_fraction(frac);
        if simplify == test {
            return true;
        }
    }

    false
}

fn simplify_fraction(input: Fraction) -> Fraction {
    let simplified = simplify(input.clone());
    if simplified != None {
        return simplified.unwrap();
    }
    input
}

fn simplify(input: Fraction) -> Option<Fraction> {
    let smaller = {
        if input.n > input.d {
            input.d
        } else {
            input.n
        }
    };
    let mut tmp = smaller;
    while tmp > 1 {
        if input.n % tmp == 0 && input.d % tmp == 0 {
            return {
                Some(Fraction {
                    n: input.n / tmp,
                    d: input.d / tmp,
                })
            };
        }
        tmp -= 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simplfy_fraction_01() {
        let check = simplify_fraction(Fraction { n: 49, d: 98 });
        let expect = Fraction { n: 1, d: 2 };
        assert_eq!(check, expect);
    }

    #[test]
    fn test_simplify_01() {
        let check = test_simplifiy(Fraction { n: 49, d: 98 });
        let expect = true;
        assert_eq!(check, expect);
    }

    #[test]
    fn test_simplify_02() {
        let check = test_simplifiy(Fraction { n: 30, d: 50 });
        let expect = false;
        assert_eq!(check, expect);
    }
}

fn main() {
    println!("Hello world");
    find_all_examples();
}
