use crate::solution::Solution;
use code_timing_macros::time_snippet;
use std::fs;

mod solution;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let input = fs::read_to_string(&"data/day4/input.txt").expect("Failed to read file.");
    let result = time_snippet!(day4::Day4::solve(&input));

    println!("{}", result)
}
