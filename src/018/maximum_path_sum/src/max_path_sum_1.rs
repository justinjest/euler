use std::fs;

fn read_file(filename: &str) -> Vec<String> {
    let tmp = open(filename);
    let mut res = Vec::new();
    for i in tmp.lines() {
        res.push(i.to_string());
    }
    return res;
}

pub fn main(){
    let a = read_file("test_path.txt");
    print!("{:?}", a);
}
