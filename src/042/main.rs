use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

fn open_file(file_name: &str) -> String {
    let path = Path::new(file_name);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read file {}: {}", display, why),
        Ok(string) => string,
    };
    s
}

fn is_coded_triangle_number(word: &str) -> bool {
    let pre: Vec<char> = word.to_uppercase().chars().collect();
    let mut sum: usize = 0;
    for i in pre {
        let val = i as usize - 64;
        sum += val as usize;
    }
    is_triangle_number(sum)
}

fn is_triangle_number(num: usize) -> bool {
    for n in 1..=num {
        if n * (n + 1) == num * 2 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_coded_triangle_number_01() {
        let check = is_coded_triangle_number("SKY");
        let expect = true;
        assert_eq!(check, expect);
    }
    #[test]
    fn is_triangle_number_01() {
        let check = is_triangle_number(55);
        let expect = true;
        assert_eq!(check, expect);
    }
}

fn main() {
    let filename = "./resources/0042_words.txt";

    let now = Instant::now();
    let contents = open_file(filename);
    let array = contents.split(',');
    let cleaned = array.into_iter().map(|x| x.replace('"', ""));
    let mut count = 0;
    for i in cleaned {
        if is_coded_triangle_number(&i) {
            println!("{} is a triangle number", i);
            count += 1;
        }
    }
    println!("Total counted: {}", count);
    let elapsed = now.elapsed();
    println!("Took {} miliseconds", elapsed.as_millis());
}
