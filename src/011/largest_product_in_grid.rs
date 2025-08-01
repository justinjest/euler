use crate::utils::files;
use std::cmp;
pub fn largest_product(filename: &str, window: usize) -> usize {
    let array = files::open_array(filename);
    let mut largest = 0;
    let len_x = array.len();
    let len_y = array[0].len();
    for y in 0..len_y - window {
            for x in 0..len_x - window {
                let val:usize = array[y][x].parse::<usize>().unwrap();
                let mut down = val;
                let mut right = val;
                let mut diagonal_down = val;
                let mut diagonal_up = val;
                for q in 1..window {
                    // down
                    let tmp0 = array[y+q][x].parse::<usize>().unwrap();
                    down = down * tmp0;
                    // right
                    let tmp1 = array[y][x+q].parse::<usize>().unwrap();
                    right = right * tmp1;
                    let tmp2 = array[y+q][x+q].parse::<usize>().unwrap();
                    diagonal_down = diagonal_down * tmp2;
                    // diagonal up
                    if x > window {
                        let tmp3 = array [y+q][x-q].parse::<usize>().unwrap();
                        diagonal_up = diagonal_up * tmp3;
                    }
                }
                largest = cmp::max(cmp::max(cmp::max(down, right),
                                            cmp::max(diagonal_down, diagonal_up)), largest);
            }
    }
    return largest;
}
