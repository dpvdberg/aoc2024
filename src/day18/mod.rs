use crate::solution::Solution;
use crate::utils::geometry::Direction;
use crate::utils::nalgebra::VectorHelpers;
use nalgebra::{vector, DMatrix, Vector2};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt;
use strum::IntoEnumIterator;

#[cfg(test)]
mod test;
pub struct Day18 {
    pub max_byte_count: Option<usize>,
}

type Position = Vector2<i32>;

#[derive(Debug, Clone, PartialEq)]
enum MemoryTile {
    EMPTY,
    BYTE,
}

impl fmt::Display for MemoryTile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MemoryTile::EMPTY => fmt.write_str("."),
            MemoryTile::BYTE => fmt.write_str("#"),
        }
    }
}

#[derive(Debug, Clone)]
struct MemorySpace {
    field: DMatrix<MemoryTile>,
}

#[derive(Debug, Clone)]
struct MemorySpacePath {
    space: MemorySpace,
    path: Option<Vec<Vector2<i32>>>,
}

struct FallingBytes {
    bytes: Vec<Position>,
}

impl FallingBytes {
    fn new(input: &str) -> Self {
        let bytes = input
            .trim()
            .lines()
            .map(|line| line.trim())
            .map(|line| line.split_once(",").unwrap())
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .map(|(x, y)| vector![x, y])
            .collect::<Vec<Position>>();

        Self { bytes }
    }

    fn has_path(&self, byte_count: usize) -> bool {
        let ram = MemorySpace::new(self, Some(byte_count));
        ram.find_path().path.is_some()
    }

    fn find_blocking_byte(&self) -> Option<Position> {
        let mut left: usize = 0;
        let mut right = self.bytes.len();
        
        if self.has_path(right) {
            return None
        }

        while left < right {
            let middle = (left + right) / 2;

            if self.has_path(middle + 1) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        Some(self.bytes[left].clone())
    }
}

impl MemorySpace {
    fn new(falling_bytes: &FallingBytes, take: Option<usize>) -> Self {
        let bytes = falling_bytes
            .bytes
            .iter()
            .take(take.unwrap_or(usize::MAX))
            .collect::<Vec<_>>();

        let max_x = bytes.iter().map(|v| v.x + 1).max().unwrap() as usize;
        let max_y = bytes.iter().map(|v| v.y + 1).max().unwrap() as usize;

        let mut field = DMatrix::from_element(max_y, max_x, MemoryTile::EMPTY);

        for byte in bytes {
            field[byte.to_matrix_index()] = MemoryTile::BYTE;
        }

        Self { field }
    }

    fn within_bounds(&self, p: &Position) -> bool {
        p.x >= 0 && p.x < self.field.ncols() as i32 && p.y >= 0 && p.y < self.field.nrows() as i32
    }

    fn neighbors(&self, pos: &Position) -> Vec<Position> {
        Direction::iter()
            .map(|d| pos + d.to_vector())
            .filter(|p| self.within_bounds(p))
            .filter(|p| self.field[p.to_matrix_index()] != MemoryTile::BYTE)
            .collect()
    }

    fn heuristic(position: &Position, target: &Position) -> u32 {
        (position - target).abs().sum() as u32
    }

    fn reconstruct_path(
        predecessors: &HashMap<Position, Position>,
        end: Position,
    ) -> Vec<Position> {
        let mut current = end;
        let mut path = vec![current];
        while predecessors.contains_key(&current) {
            current = predecessors[&current];
            path.push(current);
        }

        path.into_iter().rev().collect()
    }

    fn a_star(&self, start: Position, target: Position) -> Option<Vec<Position>> {
        let mut open_set: PriorityQueue<Position, Reverse<u32>> = PriorityQueue::new();
        let mut predecessor: HashMap<Position, Position> = HashMap::new();

        let mut g_score: HashMap<Position, u32> = HashMap::new();
        g_score.insert(start, 0);

        open_set.push(start, Reverse(Self::heuristic(&start, &target)));

        while !open_set.is_empty() {
            let (current_position, _) = open_set.pop().unwrap();
            let current_score = *g_score.get(&current_position).unwrap_or(&u32::MAX);

            if current_position == target {
                return Some(Self::reconstruct_path(&predecessor, target));
            }

            for neighbor in self.neighbors(&current_position) {
                let score = current_score + 1;
                if score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                    predecessor.insert(neighbor, current_position);
                    g_score.insert(neighbor, score);
                    open_set.push(neighbor, Reverse(score + Self::heuristic(&start, &target)));
                }
            }
        }

        None
    }

    fn top_left(&self) -> Position {
        vector![0, 0]
    }
    fn bottom_right(&self) -> Position {
        vector![
            (self.field.ncols() - 1) as i32,
            (self.field.nrows() - 1) as i32
        ]
    }

    fn find_path(&self) -> MemorySpacePath {
        MemorySpacePath {
            space: self.clone(),
            path: self.a_star(self.top_left(), self.bottom_right()),
        }
    }
}

impl fmt::Display for MemorySpace {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.field.nrows() {
            for col in 0..self.field.ncols() {
                write!(fmt, "{}", self.field[(row, col)])?;
            }
            write!(fmt, "\n")?;
        }

        Ok(())
    }
}

impl fmt::Display for MemorySpacePath {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let path = self.path.clone().unwrap_or(vec![]);
        for row in 0..self.space.field.nrows() {
            for col in 0..self.space.field.ncols() {
                let v: Position = vector![col as i32, row as i32];
                if path.contains(&v) {
                    write!(fmt, "O")?;
                } else {
                    write!(fmt, "{}", self.space.field[v.to_matrix_index()])?;
                }
            }
            write!(fmt, "\n")?;
        }

        Ok(())
    }
}

impl Solution for Day18 {
    fn solve_part1(&self, input: &str) -> String {
        let bytes = FallingBytes::new(input);
        let ram = MemorySpace::new(&bytes, self.max_byte_count);
        let path = ram.find_path();

        println!("{}", path);

        // Note: the solution does not take the first location into account
        (path.path.unwrap().len() - 1).to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let bytes = FallingBytes::new(input);
        let blocking_byte = bytes.find_blocking_byte().unwrap();
        format!("{},{}", blocking_byte.x, blocking_byte.y)
    }
}
