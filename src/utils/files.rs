use std::fs;

pub fn open(filename: &str) -> String {
   let mut contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    contents.retain(|c| !c.is_whitespace());
    return contents;
}
