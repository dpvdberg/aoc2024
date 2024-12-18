use crate::solution::Solution;
#[cfg(test)]
mod test;
pub struct Day2 {}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .skip_while(|&x| x.is_empty())
        .map(|line| {
            let split = line.trim().split_whitespace();
            let numbers = split
                .map(|number| number.parse::<i32>().expect("Could not parse number"))
                .collect::<Vec<i32>>();
            numbers
        })
        .collect()
}

fn is_safe(numbers: &Vec<i32>) -> bool {
    numbers.windows(2).all(|w| {
        let diff = w[1] - w[0];
        1 <= diff && diff <= 3
    })
}

fn is_safe_with_removal(numbers: &Vec<i32>) -> bool {
    if is_safe(numbers) {
        return true;
    }

    (0..numbers.len())
        .map(|i| {
            let mut vec = numbers.clone();
            vec.remove(i);
            vec
        })
        .any(|vec| is_safe(&vec))
}

fn is_safe_bidirectional(numbers: &Vec<i32>, allow_removal: bool) -> bool {
    let reversed = &numbers.iter().rev().cloned().collect();
    if allow_removal {
        is_safe_with_removal(&numbers) || is_safe_with_removal(reversed)
    } else {
        is_safe(&numbers) || is_safe(reversed)
    }
}

fn count_safe_reports(input: &str, allow_removal: bool) -> usize {
    let reports = parse_input(input);

    reports
        .iter()
        .filter(|r| is_safe_bidirectional(&r, allow_removal))
        .count()
}

impl Solution for Day2 {
    fn solve_part1(&self, input: &str) -> String {
        count_safe_reports(input, false).to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        count_safe_reports(input, true).to_string()
    }
}
