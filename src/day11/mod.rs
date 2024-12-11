use crate::solution::Solution;
use memoize::memoize;

#[cfg(test)]
mod test;
pub struct Day11 {}

struct PlutonianPebbles {
    pebbles: Vec<u64>,
}

fn replace_if_zero(pebble: u64) -> Option<u64> {
    if pebble == 0 {
        return Some(1)
    }

    None
}

fn split_even_length(pebble: u64) -> Option<(u64, u64)> {
    let length = pebble.checked_ilog10().unwrap_or(0) + 1;
    if length % 2 == 0 {
        // even length pebble engraving
        let half = length / 2;
        let divisor: u64 = 10_u64.pow(half);
        let left = pebble / divisor;
        let right = pebble % divisor;

        // replace current with left
        return Some((left, right))
    }

    None
}

fn multiply(pebble: u64) -> u64 {
    pebble * 2024
}

fn change_stone(pebble: u64) -> Vec<u64> {
    if let Some(new_stone) = replace_if_zero(pebble) {
        return vec![new_stone];
    }

    if let Some(new_stones) = split_even_length(pebble) {
        return vec![new_stones.0, new_stones.1];
    }

    vec![multiply(pebble)]
}

#[memoize]
fn blink_pebble(pebble: u64, count: u32) -> u64 {
    let new_pebbles = change_stone(pebble);
    if count == 1 {
        // println!("{}", new_pebbles.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" "));
        return new_pebbles.len() as u64
    }

    new_pebbles.iter().map(|p| blink_pebble(*p, count - 1)).sum()
}

impl PlutonianPebbles {
    fn blink(&self, count: u32) -> u64 {
        self.pebbles.iter().map(|p| blink_pebble(*p, count)).sum()
    }
}

fn parse_input(input: &str) -> PlutonianPebbles {
    let pebbles = input
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    PlutonianPebbles { pebbles }
}

impl Solution for Day11 {
    fn solve_part1(input: &str) -> String {
        let plutonian_pebbles = parse_input(&input);
        plutonian_pebbles.blink(25).to_string()
    }

    fn solve_part2(input: &str) -> String {
        let plutonian_pebbles = parse_input(&input);
        plutonian_pebbles.blink(75).to_string()
    }
}
