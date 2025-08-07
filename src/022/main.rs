use std::fs;

fn get_list(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    let mut items:Vec<String> = contents.split(',')
        .map(|s| s.trim_matches('"').to_string())
        .collect();
    items.sort();
    items
}

fn get_value_from_name(name: String) -> i32 {
    let mut val = 0;
    let mut chars = name.chars();
    for char_in in chars {
        val += char_in as i32 - 64;
        println!("{val}");
    }
    val
}

fn main() {
    let mut names:Vec<String> = get_list("resources/names.txt");
    let mut value = 0;
    for name in 0..names.len() {
        value += get_value_from_name(names[name].to_string()) * (name as i32 + 1);
        println!("{value}");
    };
}
