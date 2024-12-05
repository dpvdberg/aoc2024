use crate::solution::Solution;

#[cfg(test)]
mod test;
pub struct Day5 {}

#[derive(Debug)]
struct PageData {
    ordering_rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

fn parse_ordering_rules(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

fn parse_update_pages(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.split(',').map(|e| e.parse::<u32>().unwrap()).collect())
        .collect()
}

fn parse_input(input: &str) -> PageData {
    let normalized = input.replace("\r\n", "\n");
    let lines: Vec<_> = normalized.lines().map(|l| l.trim()).collect();
    let processed_input = lines.join("\n");
    let mut parts = processed_input.split("\n\n");

    let raw_ordering_rules = parts.next().unwrap().trim();
    let raw_update_pages = parts.next().unwrap().trim();

    let data = PageData {
        ordering_rules: parse_ordering_rules(raw_ordering_rules),
        updates: parse_update_pages(raw_update_pages),
    };

    data
}

fn satisfies_ordering_rule(update: &Vec<u32>, rule: &(u32, u32)) -> bool {
    if !update.contains(&rule.0) || !update.contains(&rule.1) {
        return true
    }

    let left_index = update.iter().position(|&n| n == rule.0).unwrap();
    let right_index = update.iter().position(|&n| n == rule.1).unwrap();

    left_index < right_index
}

fn update_satisfies_ordering_rules(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    rules
        .iter()
        .all(|r| satisfies_ordering_rule(update, r))
}

fn create_update_for_rules(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut new_update = update.clone();

    while let Some((left, right)) = rules.iter().find(|r| !satisfies_ordering_rule(&new_update, r)) {
        let left_index = new_update.iter().position(|&n| n == *left).unwrap();
        let right_index = new_update.iter().position(|&n| n == *right).unwrap();
        new_update.swap(left_index, right_index);
    }
    
    new_update
}

fn get_middle_number(update: &Vec<u32>) -> u32 {
    if update.len() % 2 == 0 {
        panic!("Found update page with even length, cannot find middle number")
    }

    update[(update.len() - 1) / 2]
}

impl Solution for Day5 {
    fn solve_part1(input: &str) -> String {
        let data = parse_input(input);

        data.updates
            .iter()
            .filter(|&p| update_satisfies_ordering_rules(p, &data.ordering_rules))
            .map(|p| get_middle_number(&p))
            .sum::<u32>()
            .to_string()
    }

    fn solve_part2(input: &str) -> String {
        let data = parse_input(input);
        
        data.updates
            .iter()
            .filter(|&p| !update_satisfies_ordering_rules(p, &data.ordering_rules))
            .map(|p| create_update_for_rules(p, &data.ordering_rules))
            .map(|p| get_middle_number(&p))
            .sum::<u32>()
            .to_string()
    }
}
