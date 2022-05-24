use std::cmp::{max, min};

pub fn square_of_sum(n: u32) -> u32 {
    let x: u32 = ((0..n + 1).sum());
    x.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (0..n + 1).map(|x| x.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    max(sum_of_squares(n), square_of_sum(n)) - min(sum_of_squares(n), square_of_sum(n))
}
