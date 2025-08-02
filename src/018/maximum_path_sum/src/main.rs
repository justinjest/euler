
use std::fs::read_to_string;

struct Node {
    val: u32,
    lLeg: Option<Box<Node>>,
    rLeg: Option<Box<Node>>
}

fn max_u32(a: u32, b: u32) -> u32 {
    if a > b {
        return a;
    }
    b
}

fn read_file(filename: &str) -> Vec<u32> {
    let mut tmp = read_to_string(filename).expect("Unable to open file");
    tmp = tmp.replace("\n", " ");
    let nums: Vec<&str>  = tmp.split(' ').collect();
    let mut num_vec: Vec<u32> = nums.into_iter()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.parse::<u32>().unwrap_or_default())
        .collect();

    let mut row = 0;
    while (row * (row + 1)) / 2 < num_vec.len() - 1 {
        // This should be all of the items in the array without overflow
        let row_start = (row * (row + 1)) / 2;
        let row_end = ((row + 1) * (row + 2)) / 2;
        let mut l_parent = 0;
        let mut r_parent = 0;
        for i in row_start..row_end {
            print!("child: {:?}\n", num_vec[i]);
            if row_start == i {
                l_parent = 0;
            } else {
                l_parent = num_vec[i - row - 1];
            }
            if row_end == i + 1 {
                r_parent = 0;
            } else {
                r_parent = num_vec[i - row];
            }
            print!("parents: {:?}\n", (l_parent, r_parent));
            let child = num_vec[i];
            num_vec[i] = max_u32((child + l_parent), (child + r_parent));
        }
        print!("\n");
        row = row + 1;
    }

     num_vec
}

pub fn main(){
    let a = read_file("resources/0067_triangle.txt");
    print!("{:?}\n", a.iter().max().unwrap());
}

