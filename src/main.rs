use crate::solution::Solution;
use code_timing_macros::time_snippet;
use std::fs;

pub mod day1;
pub mod day2;
mod solution;

fn main() {
    let input = fs::read_to_string(&"data/day2/input.txt").expect("Failed to read file.");
    let result = time_snippet!(day2::Day2::solve(&input));

    println!("{}", result)
}
