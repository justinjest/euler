use crate::utils::factor;

pub fn create_triangular_number (input: usize) -> usize {
    return (input * (input + 1)) / 2;
}

pub fn find_highly_divisable_triangle_number(num_of_factors: usize) -> usize {
    let mut num = 1;
    let mut triangle_num = create_triangular_number(num);
    let mut res = 1;
    while res <= num_of_factors {
        triangle_num = create_triangular_number(num);
        res = factor::factorization(triangle_num).len();
        num += 1;
    }
    return triangle_num;
}
