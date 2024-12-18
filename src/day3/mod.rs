use crate::solution::Solution;
use regex::Regex;
use std::collections::HashMap;
#[cfg(test)]
mod test;
pub struct Day3 {}

static MUL_REGEX: &str = r"mul\((\d+),(\d+)\)";

fn extract_values(input: &str, re: Regex) -> Vec<(i32, i32)> {
    let mut results: Vec<(i32, i32)> = vec![];
    for (_, [left, right]) in re.captures_iter(input.trim()).map(|c| c.extract()) {
        results.push((left.parse().unwrap(), right.parse().unwrap()));
    }

    results
}

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(MUL_REGEX).unwrap();
    extract_values(input, re)
}
enum Action {
    EnableMul,
    DisableMul,
    ApplyMul,
}

impl Solution for Day3 {
    fn solve_part1(&self, input: &str) -> String {
        parse_input(input)
            .iter()
            .map(|&(left, right)| left * right)
            .sum::<i32>()
            .to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut action_map: HashMap<&str, Action> = HashMap::new();

        action_map.insert(r"do\(\)", Action::EnableMul);
        action_map.insert("don't", Action::DisableMul);
        action_map.insert(MUL_REGEX, Action::ApplyMul);

        let mut actions = action_map
            .iter()
            .map(|(needle, action)| {
                let re = Regex::new(needle).unwrap();
                return re
                    .find_iter(input)
                    .map(|m| (action, m.as_str(), m.start()))
                    .collect::<Vec<(&Action, &str, usize)>>();
            })
            .into_iter()
            .flatten()
            .collect::<Vec<(&Action, &str, usize)>>();
        actions.sort_by_key(|&(_, _, p)| p);

        let mut enable = true;
        let mut sum = 0;

        let mut apply_action = |action: &Action, s: &str| match action {
            Action::EnableMul => enable = true,
            Action::DisableMul => enable = false,
            Action::ApplyMul => {
                if enable {
                    let re = Regex::new(MUL_REGEX).unwrap();
                    let m = re.captures(s).unwrap();
                    let left = m
                        .get(1)
                        .map(|m| m.as_str().parse::<i32>().unwrap())
                        .unwrap();
                    let right = m
                        .get(2)
                        .map(|m| m.as_str().parse::<i32>().unwrap())
                        .unwrap();
                    sum += left * right;
                }
            }
        };

        actions.iter().for_each(|(a, s, _)| apply_action(a, *s));

        sum.to_string()
    }
}
