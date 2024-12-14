use std::collections::{HashSet, VecDeque};
use crate::solution::Solution;
use nalgebra::{DMatrix, Vector2};
use once_cell::sync::Lazy;
use crate::utils::geometry::DIRECTION_VECTORS;

#[cfg(test)]
mod test;
pub struct Day10 {}

struct TopologicalMap {
    heights: DMatrix<i32>,
}

impl TopologicalMap {
    fn create_map_index(location: &Vector2<i32>) -> (usize, usize) {
        (location.y as usize, location.x as usize)
    }
    fn get_trailheads(&self) -> Vec<Vector2<i32>> {
        let mut trailheads: Vec<Vector2<i32>> = vec![];

        for y in 0..self.heights.nrows() {
            for x in 0..self.heights.ncols() {
                let v: Vector2<i32> = Vector2::new(x as i32, y as i32);
                if self.heights[Self::create_map_index(&v)] == 0 {
                    trailheads.push(v);
                }
            }
        }

        trailheads
    }
    
    fn within_bounds(&self, l: &Vector2<i32>) -> bool {
        l.x >= 0
            && l.x < self.heights.ncols() as i32
            && l.y >= 0
            && l.y < self.heights.nrows() as i32
    }

    fn get_neighbors(&self, location: &Vector2<i32>) -> Vec<Vector2<i32>> {
        let current_height = self.heights[Self::create_map_index(location)];

        DIRECTION_VECTORS
            .iter()
            .map(|d| location + d)
            .filter(|l| self.within_bounds(l))
            .filter(|l| self.heights[Self::create_map_index(l)] == 1 + current_height)
            .collect::<Vec<Vector2<i32>>>()
    }

    fn bfs_walk(&self, location: &Vector2<i32>, allow_revisit: bool) -> i32 {
        let mut queue : VecDeque<Vector2<i32>> = VecDeque::new();
        let mut visited : HashSet<Vector2<i32>> = HashSet::new();

        let mut score = 0;

        visited.insert(location.clone());
        queue.push_back(location.clone());

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();

            for neighbor in self.get_neighbors(&current) {
                if allow_revisit || !visited.contains(&neighbor) {
                    visited.insert(neighbor.clone());

                    let height = self.heights[Self::create_map_index(&neighbor)];
                    if height == 9 {
                        score += 1;
                    }

                    queue.push_back(neighbor.clone());
                }
            }
        }

        score
    }
    
    fn score(&self) -> i32 {
        self.get_trailheads()
            .iter()
            .map(|t| self.bfs_walk(t, false))
            .sum()
    }

    fn rating(&self) -> i32 {
        self.get_trailheads()
            .iter()
            .map(|t| self.bfs_walk(t, true))
            .sum()
    }
}

fn parse_input(input: &str) -> TopologicalMap {
    let heights: Vec<Vec<i32>> = input
        .trim()
        .lines()
        .map(|l| l.trim())
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).map(|d| d as i32).unwrap_or(-1))
                .collect()
        })
        .collect();

    let rows = heights.len();
    let columns = heights.first().map_or(0, |l| l.len());

    if heights.iter().any(|l| l.len() != columns) {
        panic!("Not all lines have the same length.")
    }

    let flattened = heights
        .iter()
        .flatten()
        .map(|c| c.clone())
        .collect::<Vec<i32>>();

    TopologicalMap {
        heights: DMatrix::from_row_iterator(rows, columns, flattened),
    }
}

impl Solution for Day10 {
    fn solve_part1(input: &str) -> String {
        let map = parse_input(input);
        map.score().to_string()
    }

    fn solve_part2(input: &str) -> String {
        let map = parse_input(input);
        map.rating().to_string()
    }
}
