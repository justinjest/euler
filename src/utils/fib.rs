pub fn next_fibonachi_number(i1: usize, i2: usize) -> usize {
    return i1 + i2;
}
#[allow(dead_code)]
pub fn fib_sequence(len: usize) -> Vec<usize> {
    let mut res = Vec::new();
    res.push(1);
    res.push(1);
    for n in 1..len{
       res.push(next_fibonachi_number(res[n], res[n-1]));
    }
    return res;
}
#[allow(dead_code)]
pub fn fib_sequence_max(val: usize) -> Vec<usize> {
    let mut res = Vec::new();
    res.push(1);
    res.push(1);
    let mut tmp = 0;
    while tmp <= val {
        let last = res.len() - 1;
        tmp = next_fibonachi_number(res[last], res[last-1]);
        if tmp > val {
            break;
        }
        res.push(tmp);
    }
    return res;
}
