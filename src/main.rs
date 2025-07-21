#![allow(dead_code)]

mod utils;
mod largest_product_in_grid;
fn main() {
    let res = largest_product_in_grid::largest_product("./resources/largest_product_in_a_grid.txt", 4);
    print!("{res}");
}
