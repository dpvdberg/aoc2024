use crate::solution::Solution;
use nalgebra::{vector, DMatrix, Vector2};
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[cfg(test)]
mod test;
pub struct Day6 {}

#[derive(Clone, PartialEq, Debug)]
enum TileType {
    Wall,
    Floor,
    Start,
}

impl fmt::Display for TileType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TileType::Wall => fmt.write_str("#"),
            TileType::Floor => fmt.write_str("."),
            TileType::Start => fmt.write_str("^"),
        }
    }
}

static TILE_TYPE_STRING: Lazy<HashMap<char, TileType>> = Lazy::new(|| {
    [
        (TileType::Wall.to_string().chars().next().unwrap(), TileType::Wall),
        (TileType::Floor.to_string().chars().next().unwrap(), TileType::Floor),
        (TileType::Start.to_string().chars().next().unwrap(), TileType::Start),
    ]
        .iter()
        .cloned()
        .collect()
});

impl fmt::Display for Guard {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.field.nrows() {
            for col in 0..self.field.ncols() {
                if self.history.iter()
                    .map(|h| h.position)
                    .collect::<Vec<_>>()
                    .contains(&Vector2::new(col as i32, row as i32)) {
                    write!(fmt, "@")?;
                } else {
                    let tile = self.field.get((row, col)).unwrap();
                    write!(fmt, "{}", tile)?;
                }
            }
            write!(fmt, "\n")?;
        }

        Ok(())
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Location {
    position: Vector2<i32>,
    direction: Direction,
}

fn to_field_index(location: &Vector2<i32>) -> (usize, usize) {
    (location.y as usize, location.x as usize)
}

fn is_on_field(field: &DMatrix<TileType>, location: &Vector2<i32>) -> bool {
    field.get(to_field_index(location)).is_some()
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

trait RotatableDirection {
    fn rotate_right(&self) -> Direction;
}

trait VectorizedDirection {
    fn to_vector(&self) -> Vector2<i32>;
}

impl RotatableDirection for Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl VectorizedDirection for Direction {
    fn to_vector(&self) -> Vector2<i32> {
        match self {
            Direction::Up => vector![0, -1],
            Direction::Down => vector![0, 1],
            Direction::Left => vector![-1, 0],
            Direction::Right => vector![1, 0],
        }
    }
}

fn find_position<T: PartialEq>(matrix: &DMatrix<T>, target: &T) -> Option<Vector2<i32>> {
    for row in 0..matrix.nrows() {
        for col in 0..matrix.ncols() {
            if matrix[(row, col)] == *target {
                return Some(vector![col as i32, row as i32]);
            }
        }
    }
    None
}

struct Guard {
    field: DMatrix<TileType>,
    history: HashSet<Location>,
}

fn walk_grid(field: &DMatrix<TileType>) -> Option<HashSet<Vector2<i32>>> {
    let mut guard = Guard {
        field: field.clone(),
        history: HashSet::new(),
    };

    let mut guard_position = find_position(&guard.field, &TileType::Start).unwrap();
    let mut guard_direction = Direction::Up;

    guard.history.insert(Location {
        position: guard_position,
        direction: guard_direction.clone(),
    });

    let mut new_position = guard_position + guard_direction.to_vector();

    while is_on_field(&guard.field, &new_position) {
        if guard.field[to_field_index(&new_position)] == TileType::Wall {
            guard_direction = guard_direction.rotate_right();
        } else {
            guard_position = new_position;
        }

        if !guard.history.insert(Location {
            position: guard_position,
            direction: guard_direction.clone(),
        }) {
            return None;
        }

        new_position = guard_position + guard_direction.to_vector()
    }

    let unique_positions: HashSet<Vector2<i32>> = guard.history.iter()
        .map(|l| l.position)
        .collect();

    // println!("{}", guard);

    Some(unique_positions)
}

fn parse_input(input: &str) -> DMatrix<TileType> {
    let tiles: Vec<Vec<TileType>> = input
        .trim()
        .lines()
        .map(|l| l.trim())
        .map(|l| l.chars().map(|c| TILE_TYPE_STRING[&c].clone()).collect())
        .collect();

    let rows = tiles.len();
    let columns = tiles.first().map_or(0, |l| l.len());

    if tiles.iter().any(|l| l.len() != columns) {
        panic!("Not all lines have the same length.")
    }

    let flattened = tiles
        .iter()
        .flatten()
        .map(|c| c.clone())
        .collect::<Vec<TileType>>();
    DMatrix::from_row_iterator(rows, columns, flattened)
}

impl Solution for Day6 {
    fn solve_part1(input: &str) -> String {
        let field = parse_input(input);
        walk_grid(&field).unwrap().len().to_string()
    }

    fn solve_part2(input: &str) -> String {
        let field = parse_input(input);
        let initial_positions = walk_grid(&field).unwrap_or_default();

        initial_positions.iter()
            .filter(|p| field[to_field_index(p)] == TileType::Floor)
            .filter(|p| {
                let mut modified_grid = field.clone();
                modified_grid[to_field_index(p)] = TileType::Wall;
                
                walk_grid(&modified_grid).is_none()
            })
            .count()
            .to_string()
    }
}
