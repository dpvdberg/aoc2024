use std::collections::VecDeque;
use std::fmt;
use nalgebra::{vector, DMatrix, Vector2};
use crate::solution::Solution;
use crate::utils::geometry::{Direction};
use crate::utils::nalgebra::{MatrixHelpers, VectorHelpers, MatrixParser};

#[cfg(test)]
mod test;
pub struct Day15 {}

#[derive(Clone, Debug, PartialEq)]
enum WarehouseTile {
    Wall,
    Box,
    Floor,
    BoxLeft,
    BoxRight,
    Robot
}
impl fmt::Display for WarehouseTile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WarehouseTile::Wall => fmt.write_str("#"),
            WarehouseTile::Box => fmt.write_str("O"),
            WarehouseTile::Floor => fmt.write_str("."),
            WarehouseTile::Robot => fmt.write_str("@"),
            WarehouseTile::BoxLeft => fmt.write_str("["),
            WarehouseTile::BoxRight => fmt.write_str("]"),
        }
    }
}

struct Warehouse {
    tiles: DMatrix<WarehouseTile>,
    robot_position: Vector2<i32>,
    remaining_moves: VecDeque<Direction>
}

impl Warehouse {
    fn box_positions(&self) -> Vec<Vector2<i32>> {
        (0..self.tiles.ncols())
            .flat_map(|x| (0..self.tiles.nrows()).map(move |y| vector![x as i32, y as i32]))
            .filter(|v : &Vector2<i32>| *self.tiles.at(v) == WarehouseTile::Box || *self.tiles.at(v) == WarehouseTile::BoxLeft)
            .collect::<Vec<Vector2<i32>>>()
    }
    
    fn move_to_two_sided(&mut self, to: Vector2<i32>, direction: &Direction, other_side: &Direction, apply: bool) -> bool {
        let this_can_move = self.move_into(to, direction, false);

        if direction.is_vertical() {
            let other_to = to + other_side.to_vector();
            let other_can_move = self.move_into(other_to, direction, false);
            
            if this_can_move && other_can_move {
                self.move_into(to, direction, apply);
                self.move_into(other_to, direction, apply);
            } else {
                return false;
            }
        } else {
            if !self.move_into(to, direction, apply) {
                return false;
            }
        }
        
        true
    }

    fn move_into(&mut self, from: Vector2<i32>, direction: &Direction, apply: bool) -> bool {
        let to = from + direction.to_vector();
        match self.tiles.at(&to) {
            WarehouseTile::Wall => {
                return false;
            }
            WarehouseTile::Box => {
                if !self.move_into(to, direction, true) {
                    return false;
                }
            }
            WarehouseTile::BoxLeft => {
                if !self.move_to_two_sided(to, direction, &Direction::Right, apply) {
                    return false;
                }
            }
            WarehouseTile::BoxRight => {
                if !self.move_to_two_sided(to, direction, &Direction::Left, apply) {
                    return false;
                }
            }
            WarehouseTile::Floor => {}
            WarehouseTile::Robot => {
                panic!("Found robot during moving")
            }
        }

        if apply {
            self.tiles[to.to_matrix_index()] = self.tiles[from.to_matrix_index()].clone();
            self.tiles[from.to_matrix_index()] = WarehouseTile::Floor;
        }

        true
    }

    fn robot_step(&mut self, direction: &Direction) -> bool {
        if self.move_into(self.robot_position, direction, true) {
            self.robot_position = self.robot_position + direction.to_vector();
            return true;
        }

        false
    }

    fn robot_walk(&mut self) {
        while let Some(direction) = self.remaining_moves.pop_front() {
            self.robot_step(&direction);

            // println!("Direction: {:?}", direction);
            // println!("{}", self)
        }
    }
}



impl fmt::Display for Warehouse {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.tiles.nrows() {
            for col in 0..self.tiles.ncols() {
                let tile = self.tiles.get((row, col)).unwrap();
                write!(fmt, "{}", tile)?;
            }
            write!(fmt, "\n")?;
        }

        Ok(())
    }
}

fn parse_warehouse(input: &str, expand: bool) -> DMatrix<WarehouseTile> {
    let mut input_processed: String = input.into();
    if expand {
        input_processed = input_processed.replace("#", "##");
        input_processed = input_processed.replace("O", "[]");
        input_processed = input_processed.replace(".", "..");
        input_processed = input_processed.replace("@", "@.");
    }
    
    let tiles = input_processed.to_matrix(|c| match c {
        '#' => WarehouseTile::Wall,
        'O' => WarehouseTile::Box,
        '.' => WarehouseTile::Floor,
        '@' => WarehouseTile::Robot,
        '[' => WarehouseTile::BoxLeft,
        ']' => WarehouseTile::BoxRight,
        _ => panic!("Could not parse warehouse tile: {}", c)
    });

    tiles
}

fn parse_moves(input: &str) -> VecDeque<Direction> {
    input.chars()
        .filter(|&c| c != '\n' && c != '\r')
        .map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("Could not parse move: {}", c)
        }).collect()
}

fn parse_input(input: &str, expand: bool) -> Warehouse {
    let normalized = input.replace("\r\n", "\n");
    let lines: Vec<_> = normalized.lines().map(|l| l.trim()).collect();
    let processed_input = lines.join("\n");
    let mut parts = processed_input.split("\n\n");

    let raw_warehouse = parts.next().unwrap().trim();
    let raw_robot_movement = parts.next().unwrap().trim();

    let warehouse = parse_warehouse(raw_warehouse, expand);
    let pos = (0..warehouse.ncols())
        .flat_map(|x| (0..warehouse.nrows()).map(move |y| vector![x as i32, y as i32]))
        .find(|v : &Vector2<i32>| *warehouse.at(v) == WarehouseTile::Robot).unwrap();

    Warehouse {
        tiles: warehouse,
        robot_position: pos,
        remaining_moves: parse_moves(raw_robot_movement)
    }
}

impl Solution for Day15 {
    fn solve_part1(input: &str) -> String {
        let mut warehouse = parse_input(input, false);
        warehouse.robot_walk();
        warehouse.box_positions().iter().map(|p| 100 * p.y + p.x).sum::<i32>().to_string()
    }

    fn solve_part2(input: &str) -> String {
        let mut warehouse = parse_input(input, true);
        warehouse.robot_walk();
        warehouse.box_positions().iter().map(|p| 100 * p.y + p.x).sum::<i32>().to_string()
    }
}
