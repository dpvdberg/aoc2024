use crate::solution::Solution;
use code_timing_macros::time_snippet;
use std::fs;

mod day1;
mod day2;
mod day3;
mod solution;

fn main() {
    let input = fs::read_to_string(&"data/day3/input.txt").expect("Failed to read file.");
    let result = time_snippet!(day3::Day3::solve(&input));

    println!("{}", result)
}
