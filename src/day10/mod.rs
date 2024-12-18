use crate::solution::Solution;
use crate::utils::geometry::Direction;
use crate::utils::nalgebra::{MatrixParser, VectorHelpers};
use nalgebra::{DMatrix, Vector2};
use std::collections::{HashSet, VecDeque};
use strum::IntoEnumIterator;

#[cfg(test)]
mod test;
pub struct Day10 {}

struct TopologicalMap {
    heights: DMatrix<i32>,
}

impl TopologicalMap {
    fn get_trailheads(&self) -> Vec<Vector2<i32>> {
        let mut trailheads: Vec<Vector2<i32>> = vec![];

        for y in 0..self.heights.nrows() {
            for x in 0..self.heights.ncols() {
                let v: Vector2<i32> = Vector2::new(x as i32, y as i32);
                if self.heights[v.to_matrix_index()] == 0 {
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
        let current_height = self.heights[location.to_matrix_index()];

        Direction::iter()
            .map(|d| location + d.to_vector())
            .filter(|l| self.within_bounds(l))
            .filter(|l| self.heights[l.to_matrix_index()] == 1 + current_height)
            .collect::<Vec<Vector2<i32>>>()
    }

    fn bfs_walk(&self, location: &Vector2<i32>, allow_revisit: bool) -> i32 {
        let mut queue: VecDeque<Vector2<i32>> = VecDeque::new();
        let mut visited: HashSet<Vector2<i32>> = HashSet::new();

        let mut score = 0;

        visited.insert(location.clone());
        queue.push_back(location.clone());

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();

            for neighbor in self.get_neighbors(&current) {
                if allow_revisit || !visited.contains(&neighbor) {
                    visited.insert(neighbor.clone());

                    let height = self.heights[neighbor.to_matrix_index()];
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
    let heights = input
        .to_string()
        .to_matrix(|c| c.to_digit(10).map(|d| d as i32).unwrap_or(-1));

    TopologicalMap { heights }
}

impl Solution for Day10 {
    fn solve_part1(&self, input: &str) -> String {
        let map = parse_input(input);
        map.score().to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let map = parse_input(input);
        map.rating().to_string()
    }
}
