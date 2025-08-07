
fn next_iteration (num: &str) -> String {
    let mut vec = Vec::new();
    for c in num.chars() {
        vec.push(c.to_digit(10).expect("Not a number"));
    }

    let mut k = 0;
    for i in 0..vec.len() - 1 {
        if vec[i] < vec[i + 1] {
            k = i;
        }
    }

    let mut l = 0;
    for i in 0..vec.len() {
        if vec[k] < vec[i]{
            l = i;
        }
    }

    vec.swap(k, l);
    vec[k+1..].reverse();
    let output = vec.iter()
        .map(|a| a.to_string())
        .collect::<Vec<_>>()
        .join("");
    return output;
}

fn main() {
    let mut num = "0123456789".to_string();
    println!("{num}");
    for i in 1..1_000_000 {
        num = next_iteration(&num);
        println!("{num}");
    }
}
