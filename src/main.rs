use crate::solution::Solution;
use code_timing_macros::time_snippet;
use std::fs;

mod solution;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let input = fs::read_to_string(&"data/day5/input.txt").expect("Failed to read file.");
    let result = time_snippet!(day5::Day5::solve(&input));

    println!("{}", result)
}
