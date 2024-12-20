use crate::solution::Solution;
use crate::utils::geometry::Direction;
use crate::utils::nalgebra::{MatrixHelpers, MatrixParser, VectorHelpers};
use nalgebra::{DMatrix, Vector2};
use rayon::{iter::IntoParallelIterator, iter::ParallelIterator};
use std::collections::{HashMap, HashSet};
use std::fmt;
use strum::IntoEnumIterator;

#[cfg(test)]
mod test;
pub struct Day20 {}

#[derive(Debug, Clone, PartialEq)]
enum RaceTile {
    Start,
    End,
    Floor,
    Wall,
}

impl fmt::Display for RaceTile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RaceTile::Start => fmt.write_str("S")?,
            RaceTile::End => fmt.write_str("E")?,
            RaceTile::Floor => fmt.write_str(".")?,
            RaceTile::Wall => fmt.write_str("#")?,
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct RaceTrack {
    field: DMatrix<RaceTile>,
}

type Position = Vector2<i32>;

struct RaceTrackPath {
    track: RaceTrack,
    path: Vec<Position>,
}

impl RaceTrack {
    fn within_bounds(&self, p: &Position) -> bool {
        p.x >= 0 && p.x < self.field.ncols() as i32 && p.y >= 0 && p.y < self.field.nrows() as i32
    }

    fn neighbors(&self, pos: &Position, tile_types: Vec<RaceTile>) -> Vec<Position> {
        Direction::iter()
            .map(|d| pos + d.to_vector())
            .filter(|p| self.within_bounds(p))
            .filter(|p| tile_types.contains(&self.field[p.to_matrix_index()]))
            .collect()
    }

    fn new(input: &str) -> Self {
        let field = input.to_string().to_matrix(|c| match c {
            'S' => RaceTile::Start,
            'E' => RaceTile::End,
            '.' => RaceTile::Floor,
            '#' => RaceTile::Wall,
            _ => panic!("Could not match race tile '{c}'"),
        });
        Self { field }
    }

    fn start_position(&self) -> Position {
        self.field.find_index(RaceTile::Start).unwrap()
    }

    fn end_position(&self) -> Position {
        self.field.find_index(RaceTile::End).unwrap()
    }

    fn step(&self, position: &Position, history: &HashSet<Position>) -> Position {
        self.neighbors(position, vec![RaceTile::Floor, RaceTile::End])
            .iter()
            .filter(|p| !history.contains(p))
            .next()
            .unwrap()
            .clone()
    }

    fn find_path(&self) -> RaceTrackPath {
        let mut current = self.start_position();
        let mut path = vec![];
        let mut visited = HashSet::new();

        while current != self.end_position() {
            path.push(current);
            visited.insert(current);

            current = self.step(&current, &visited);
        }

        // Add end to path
        path.push(current);

        RaceTrackPath {
            track: self.clone(),
            path,
        }
    }

    fn manhattan_neighbors(position: &Position, distance: usize) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let int_distance = distance as i32;

        for x in -int_distance..=int_distance {
            let y = int_distance - x.abs();
            neighbors.push(position + Vector2::new(x, y));
            if y != 0 {
                neighbors.push(position + Vector2::new(x, -y));
            }
        }

        neighbors
    }

    fn find_jumps(
        &self,
        position: &Position,
        jump_size: usize,
    ) -> Vec<(Position, Position, usize)> {
        let jumps = Self::manhattan_neighbors(position, jump_size);
        let landings = jumps
            .iter()
            .filter(|p| self.within_bounds(p))
            .filter(|p| self.field[p.to_matrix_index()] != RaceTile::Wall)
            .map(|p| (position.clone(), p.clone(), jump_size))
            .collect();

        landings
    }

    fn find_cheats(&self, max_jump_size: usize) -> Vec<i32> {
        let path = self.find_path().path;

        let index_map = path
            .iter()
            .enumerate()
            .map(|(i, p)| (p, i as i32))
            .collect::<HashMap<_, _>>();

        let cheats = (1..=max_jump_size)
            .into_par_iter()
            .flat_map(|jump_size| {
                path.iter()
                    .flat_map(|p| self.find_jumps(p, jump_size))
                    .collect::<Vec<(Position, Position, usize)>>()
            })
            .map(|(start, end, jump_size)| (index_map[&start], index_map[&end], jump_size))
            .map(|(start_pos, end_pos, jump_size)| end_pos - (start_pos + jump_size as i32))
            .filter(|&s| s > 0)
            .collect();

        cheats
    }
}

impl fmt::Display for RaceTrackPath {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.track.field.nrows() {
            for col in 0..self.track.field.ncols() {
                let v = Position::new(col as i32, row as i32);
                if self.path.contains(&v) {
                    write!(fmt, "O")?;
                } else {
                    write!(fmt, "{}", self.track.field[v.to_matrix_index()])?;
                }
            }
            write!(fmt, "\n")?;
        }

        Ok(())
    }
}

impl Solution for Day20 {
    fn solve_part1(&self, input: &str) -> String {
        let track = RaceTrack::new(input);
        let cheats = track.find_cheats(2);

        cheats.iter().filter(|&&c| c >= 100).count().to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let track = RaceTrack::new(input);
        let cheats = track.find_cheats(20);

        cheats.iter().filter(|&&c| c >= 100).count().to_string()
    }
}
