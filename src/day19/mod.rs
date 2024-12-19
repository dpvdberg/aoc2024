use memoize::memoize;
use crate::solution::Solution;

#[cfg(test)]
mod test;
pub struct Day19 {}

struct TowelAndPatterns {
    towels: Vec<String>,
    patterns: Vec<String>,
}


#[memoize]
fn count_possible_patterns(towels: Vec<String>, pattern: String) -> usize {
    if pattern.is_empty() { return 1 }

    towels.iter()
        .filter_map(|towel| pattern.strip_prefix(towel))
        .map(|remaining_pattern| count_possible_patterns(towels.clone(), remaining_pattern.into()))
        .sum()
}

impl TowelAndPatterns {
    fn new(input: &str) -> Self {
        let normalized = input.trim().replace("\r\n", "\n");
        let (raw_towels, raw_patterns) = normalized.split_once("\n\n").unwrap();

        Self {
            towels: raw_towels
                .trim()
                .split(",")
                .map(|i| i.trim().to_string())
                .collect(),
            patterns: raw_patterns
                .trim()
                .lines()
                .map(|i| i.trim().to_string())
                .collect(),
        }
    }

    fn count_valid_patterns(&self) -> usize {
        self.patterns
            .iter()
            .filter(|&p| count_possible_patterns(self.towels.clone(), p.into()) > 0)
            .count()
    }

    fn count_possible_patterns(&self) -> usize {
        self.patterns
            .iter()
            .map(|p| count_possible_patterns(self.towels.clone(), p.into()))
            .sum()
    }
}

impl Solution for Day19 {
    fn solve_part1(&self, input: &str) -> String {
        let collection = TowelAndPatterns::new(input);
        collection.count_valid_patterns().to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let collection = TowelAndPatterns::new(input);
        collection.count_possible_patterns().to_string()
    }
}
