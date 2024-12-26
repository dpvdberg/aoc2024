use crate::solution::Solution;
use crate::utils::nalgebra::MatrixParser;
use itertools::Itertools;
use nalgebra::DMatrix;

#[cfg(test)]
mod test;
pub struct Day25 {}

type Schematic = DMatrix<u8>;

struct LocksAndKeys {
    locks: Vec<Schematic>,
    keys: Vec<Schematic>,
}

impl LocksAndKeys {
    fn get_locks_and_keys(items: &Vec<Schematic>) -> Self {
        Self {
            locks: items
                .iter()
                .cloned()
                .filter(|m| m.row(0).iter().all(|&value| value == 1))
                .collect_vec(),
            keys: items
                .iter()
                .cloned()
                .filter(|m| m.row(m.nrows() - 1).iter().all(|&value| value == 1))
                .collect_vec(),
        }
    }

    fn fits(lock: &Schematic, key: &Schematic) -> bool {
        let sum = lock + key;
        sum.iter().all(|&value| value <= 1)
    }

    fn count_fitting(&self) -> usize {
        self.locks
            .iter()
            .map(|l| self.keys.iter().filter(|k| Self::fits(l, k)).count())
            .sum()
    }

    fn new(items: &Vec<Schematic>) -> Self {
        Self::get_locks_and_keys(items)
    }
}

fn parse_input(input: &str) -> LocksAndKeys {
    let normalized = input.trim().replace("\r\n", "\n");
    let items = normalized
        .split("\n\n")
        .map(|e| e.trim().to_string())
        .collect_vec();
    let matrices = items
        .iter()
        .map(|e| {
            e.to_matrix(|c| match c {
                '.' => 0,
                '#' => 1,
                _ => panic!("Cannot parse {c}"),
            })
        })
        .collect_vec();

    LocksAndKeys::new(&matrices)
}

impl Solution for Day25 {
    fn solve_part1(&self, input: &str) -> String {
        let locks_and_keys = parse_input(input);
        locks_and_keys.count_fitting().to_string()
    }

    fn solve_part2(&self, _input: &str) -> String {
        "".into()
    }
}
