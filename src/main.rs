use std::fs;
use crate::solution::Solution;
use code_timing_macros::time_snippet;

pub mod day1;
mod solution;

fn main() {
    let input = fs::read_to_string(&"data/day1/input.txt")
        .expect("Failed to read file.");
    let result = time_snippet!(day1::Day1::solve(&input));

    println!("{}", result)
}
