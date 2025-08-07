use std::collections::HashMap;

fn factorization(num: u32, factorTable: &mut HashMap<u32, u32>)
                 -> &HashMap<u32, u32> {
    let mut ans: u32 = 0;
    for n in 2..num.isqrt() + 1 {
        if num % n == 0 {
            ans += n;
            ans += num / n;
        }
    }
    factorTable.insert(num, ans+1);
   factorTable
}

fn find_amicable(factor_table: &HashMap<u32, u32>) -> Vec<u32> {
    let mut ans = Vec::new();
    for key in factor_table.keys() {
        let k = key;
        let v = factor_table[&k];

        if factor_table.contains_key(&v) {
            if factor_table[&v] == *k && v != *k{
                ans.push(*k);
            }
        }
    }
    return ans;
}

fn main() {
    let mut t = HashMap::from([
        (1, 1),
    ]);
    let mut nt: HashMap<u32, u32> = HashMap::from([(1, 1)]);
    for i in 1..=10000 {
        nt = factorization(i, &mut t).clone();
    }
    println!("{:?}", nt[&220]);
    let b = find_amicable(&nt);
    println!("{:?}", b);
    let res: u32 = b.iter().sum();
    println!("{:?}", res);
}
