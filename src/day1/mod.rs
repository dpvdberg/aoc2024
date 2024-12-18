use crate::solution::Solution;
use std::collections::HashMap;
#[cfg(test)]
mod test;
pub struct Day1 {}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let numbers: Vec<_> = input
        .trim()
        .lines()
        .skip_while(|&x| x.is_empty())
        .map(|line| {
            let mut line_split = line.trim().split_whitespace();
            let a = line_split
                .next()
                .expect("Missing first number")
                .parse::<i32>()
                .expect("First value is not a valid number");
            let b = line_split
                .next()
                .expect("Missing second number")
                .parse::<i32>()
                .expect("Second value is not a valid number");

            (a, b)
        })
        .collect();

    let mut left_numbers = numbers.iter().map(|v| v.0).collect::<Vec<i32>>();
    left_numbers.sort();

    let mut right_numbers = numbers.iter().map(|v| v.1).collect::<Vec<i32>>();
    right_numbers.sort();

    (left_numbers, right_numbers)
}

impl Solution for Day1 {
    fn solve_part1(&self, input: &str) -> String {
        let (left_numbers, right_numbers) = parse_input(input);

        let differences: Vec<i32> = left_numbers
            .iter()
            .zip(right_numbers)
            .map(|(a, b)| (a - b).abs())
            .collect::<Vec<i32>>();

        let distance_sum: i32 = differences.iter().sum();

        distance_sum.to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let (left_numbers, right_numbers) = parse_input(input);

        let right_occurrences: HashMap<i32, u32> = right_numbers
            .iter()
            .map(|v| (*v, right_numbers.iter().filter(|&n| n == v).count() as u32))
            .collect();

        let similarity_score: u32 = left_numbers
            .iter()
            .map(|v| (*v as u32) * right_occurrences.get(v).unwrap_or(&0))
            .sum();

        similarity_score.to_string()
    }
}
