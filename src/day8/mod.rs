use crate::solution::Solution;
use nalgebra::Vector2;
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod test;
pub struct Day8 {}

fn get_bounds(input: &str) -> (usize, usize) {
    let lines = input.trim().lines().collect::<Vec<&str>>();
    (lines[0].trim().len(), lines.len())
}

fn parse_input(input: &str) -> HashMap<char, Vec<Vector2<i32>>> {
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut map = HashMap::<char, Vec<Vector2<i32>>>::new();

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let char: char = lines[y].trim().chars().nth(x).unwrap();

            if char != '.' {
                map.entry(char)
                    .or_insert_with(Vec::new)
                    .push(Vector2::new(x as i32, y as i32));
            }
        }
    }

    map
}

fn is_inside(v: Vector2<i32>, bounds: (usize, usize)) -> bool {
    v.x >= 0 && v.y >= 0 && v.x < bounds.0 as i32 && v.y < bounds.1 as i32
}

fn find_antinodes_directional(
    a: Vector2<i32>,
    b: Vector2<i32>,
    resonant: bool,
    bounds: (usize, usize),
) -> HashSet<Vector2<i32>> {
    let mut antinodes = HashSet::<Vector2<i32>>::new();

    let a_to_b: Vector2<i32> = b - a;
    let mut node = b + a_to_b;
    if resonant {
        antinodes.insert(b);
        while is_inside(node, bounds) {
            antinodes.insert(node);

            node = node + a_to_b;
        }
    } else {
        if is_inside(node, bounds) {
            antinodes.insert(node);
        }
    }

    antinodes
}

fn find_antinodes(
    antennas: &Vec<Vector2<i32>>,
    resonant: bool,
    bounds: (usize, usize),
) -> HashSet<Vector2<i32>> {
    let mut antinodes = HashSet::<Vector2<i32>>::new();

    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let a = antennas[i];
            let b = antennas[j];
            antinodes.extend(find_antinodes_directional(a, b, resonant, bounds));
            antinodes.extend(find_antinodes_directional(b, a, resonant, bounds));
        }
    }

    antinodes
}

fn count_unique_antinodes(
    antennas: &HashMap<char, Vec<Vector2<i32>>>,
    resonant: bool,
    bounds: (usize, usize),
) -> usize {
    antennas
        .values()
        .map(|nodes| find_antinodes(nodes, resonant, bounds))
        .collect::<Vec<HashSet<Vector2<i32>>>>()
        .into_iter()
        .fold(HashSet::new(), |mut acc, set| {
            acc.extend(set);
            acc
        })
        .len()
}

impl Solution for Day8 {
    fn solve_part1(&self, input: &str) -> String {
        let antennas = parse_input(input);
        let bounds = get_bounds(input);

        count_unique_antinodes(&antennas, false, bounds).to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let antennas = parse_input(input);
        let bounds = get_bounds(input);

        count_unique_antinodes(&antennas, true, bounds).to_string()
    }
}
