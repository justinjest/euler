use crate::utils::files;

pub fn largest_product(filename: &str, window: usize) -> usize {
    let val = files::open(filename);
    let end = val.len() - window;
    let mut res = 0;
    for i in 0..end+1 {
        let end_str = i + window;
        let tmp = &val[i..end_str];
        let val = mult_string(tmp);
        if val > res {
            res = val;
        }
    }
    return res;
}

fn mult_string(string: &str) -> usize {
    let mut res = 1;
    for value in 0..string.len() {
        let tmp: usize = string[value..value+1].trim().parse().unwrap();
        res = res * tmp;
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_largest_product() {
        let res = largest_product("./resources/largest_product_in_a_series.txt", 4);
        assert_eq!(res, 5832);
    }
}
