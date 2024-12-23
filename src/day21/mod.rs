use crate::solution::Solution;
use crate::utils::geometry::Direction;
use crate::utils::nalgebra::MatrixHelpers;
use itertools::Itertools;
use memoize::memoize;
use nalgebra::{Matrix2x3, Matrix4x3, Vector2};
use std::iter;
use strum::IntoEnumIterator;

static NA: char = '~';
#[cfg(test)]
mod test;
pub struct Day21 {}

static DIRECTIONAL_KEYPAD_LAYOUT: Matrix2x3<char> = Matrix2x3::new(NA, '^', 'A', '<', 'v', '>');
static NUMPAD_KEYPAD_LAYOUT: Matrix4x3<char> =
    Matrix4x3::new('7', '8', '9', '4', '5', '6', '1', '2', '3', NA, '0', 'A');

fn direction_to_key(direction: Direction) -> char {
    match direction {
        Direction::Up => '^',
        Direction::Down => 'v',
        Direction::Left => '<',
        Direction::Right => '>',
    }
}

#[memoize]
fn numpad_paths(start: char, end: char)  -> Vec<String> {
    keypad_paths(
        start,
        end,
        NUMPAD_KEYPAD_LAYOUT
    )
}

#[memoize]
fn dirpad_paths(start: char, end: char)  -> Vec<String> {
    keypad_paths(
        start,
        end,
        DIRECTIONAL_KEYPAD_LAYOUT
    )
}

fn keypad_paths<Z : MatrixHelpers<char> + Clone>(start: char, end: char, pad: Z) -> Vec<String> {
    let dest = pad.find_index(end).unwrap();

    let sol = pathfinding::prelude::astar_bag_collect(
        &(start, None),
        |(current, dir)| {
            let adj: Vec<_> = Direction::iter()
                .map(|d| {
                    (
                        pad.find_index(*current).unwrap() + d.to_vector(),
                        d,
                    )
                })
                .filter(|(l, _)| pad.valid_index(l))
                .map(|(l, d)| (pad.at(&l), d))
                .filter(|(c, _)| **c != NA)
                .collect();

            return adj
                .into_iter()
                .map(|(c, d)| {
                    let cost;
                    if let Some(current_dir) = dir {
                        cost = if *current_dir == d { 1 } else { 2 }
                    } else {
                        cost = 1
                    }

                    ((*c, Some(d)), cost)
                })
                .collect::<Vec<((char, Option<Direction>), u32)>>();
        },
        |(current, _)| {
            (pad.find_index(*current).unwrap() - dest)
                .abs()
                .sum() as u32
        },
        |&(current, _)| current == end,
    )
        .unwrap();

    let (paths, _) = sol;
    paths
        .into_iter()
        .map(|path| {
            path.iter()
                .filter_map(|(_, d)| d.clone())
                .map(|d| direction_to_key(d))
                .collect()
        })
        .map(|keys: String| keys + "A")
        .collect()
}

#[memoize]
fn npad(input: String, robots: usize) -> usize {
    let mut sum = 0;
    for (a, b) in iter::once('A').chain(input.chars()).tuple_windows() {
        let sequences = numpad_paths(a, b);

        sum += sequences
            .into_iter()
            .map(|s| dpad(s, robots))
            .min()
            .unwrap();
    }
    sum
}

#[memoize]
fn dpad(input: String, robots: usize) -> usize {
    let mut acc = 0;

    for (a, b) in iter::once('A').chain(input.chars()).tuple_windows() {
        let sequences = dirpad_paths(a, b);
        let depth = robots - 1;
        if depth > 0 {
            acc += sequences
                .into_iter()
                .map(|s| dpad(s, depth))
                .min()
                .unwrap();
        } else {
            acc += sequences
                .into_iter()
                .map(|s| s.len())
                .min()
                .unwrap();
        }
    }
    
    acc
}

fn type_code(input: &str, dirpad_count: usize) -> usize {
    let mut sum = 0;
    for code in input.trim().lines().map(|l| l.trim()) {
        let code_num = code.strip_suffix("A").unwrap().parse::<usize>().unwrap();
        let length = npad(code.into(), dirpad_count);
        sum += code_num * length;
    }
    sum
}

impl Solution for Day21 {
    fn solve_part1(&self, input: &str) -> String {
        type_code(input, 2).to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        type_code(input, 25).to_string()
    }
}
