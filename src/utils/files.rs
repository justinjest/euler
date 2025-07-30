use std::fs;

pub fn open(filename: &str) -> String {
   let mut contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    contents.retain(|c| !c.is_whitespace());
    return contents;
}

pub fn open_nl_breaks(filename: &str) -> Vec<String> {
    let tmp = open(filename);
    let mut res = Vec::new();
    for i in tmp.lines() {
        print!("{i}\n");
        res.push(i.to_string());
    }
    return res;
}
// TODO: Add error checking for this, what happens when we can't open the file?
pub fn open_array(filename: &str) -> Vec<Vec<String>>{
    let mut res = Vec::new();
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    let tmp = contents.lines();
    for i in tmp{
        let mut tmp_array= Vec::new();
        let tmp0 = i.split_whitespace();
        for j in tmp0 {
            tmp_array.push(j.to_string());
        }
        res.push(tmp_array);
    }
    return res;
}
