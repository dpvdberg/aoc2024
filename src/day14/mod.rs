use crate::solution::Solution;
use crate::utils::geometry::Direction;
use nalgebra::{vector, Vector2};
use regex::Regex;
use std::fmt;
use strum::IntoEnumIterator;

#[cfg(test)]
mod test;
pub struct Day14 {}

#[derive(Clone, PartialEq)]
struct RobotMovement {
    position: Vector2<i32>,
    velocity: Vector2<i32>,
}

impl RobotMovement {
    fn walk(&self, steps: usize, room_size: Vector2<i32>) -> Vector2<i32> {
        let p: Vector2<i32> = self.position + steps as i32 * self.velocity;
        let mut iter = p
            .iter()
            .zip(room_size.iter())
            .map(|(x, max)| x.rem_euclid(*max));

        Vector2::new(iter.next().unwrap(), iter.next().unwrap())
    }

    fn get_neighbors(&self) -> Vec<Vector2<i32>> {
        let neighbors = Direction::iter()
            .map(|d| self.position + d.to_vector())
            .collect::<Vec<Vector2<i32>>>();

        neighbors
    }
}

#[derive(Clone, PartialEq)]
struct RobotField {
    robots: Vec<RobotMovement>,
    size: Vector2<i32>,
}

impl RobotField {
    fn count_robots_in_quadrants(&self) -> usize {
        let center: Vector2<i32> = Vector2::new(self.size.x / 2, self.size.y / 2);
        let q1 = self
            .robots
            .iter()
            .filter(|r| r.position.x > center.x && r.position.y > center.y)
            .count();
        let q2 = self
            .robots
            .iter()
            .filter(|r| r.position.x < center.x && r.position.y > center.y)
            .count();
        let q3 = self
            .robots
            .iter()
            .filter(|r| r.position.x < center.x && r.position.y < center.y)
            .count();
        let q4 = self
            .robots
            .iter()
            .filter(|r| r.position.x > center.x && r.position.y < center.y)
            .count();

        q1 * q2 * q3 * q4
    }

    fn compute_safety_factor(&self, steps: usize) -> usize {
        let mut moved_field = self.clone();
        for robot in moved_field.robots.iter_mut() {
            robot.position = robot.walk(steps, self.size);
        }

        moved_field.count_robots_in_quadrants()
    }

    fn count_boxed_in_robots(&self) -> usize {
        self.robots
            .iter()
            .filter(|r1| {
                r1.get_neighbors()
                    .iter()
                    .all(|n| self.robots.iter().any(|r2| r2.position == *n))
            })
            .count()
    }

    fn find_christmas_tree(&self, boxed_in_threshold: f32) -> Option<usize> {
        let mut moved_field = self.clone();

        let mut walk_counter = 0;

        loop {
            for robot in moved_field.robots.iter_mut() {
                robot.position = robot.walk(1, self.size);
            }

            walk_counter += 1;

            let boxed_in_robots = moved_field.count_boxed_in_robots();
            let boxed_in_percentage = boxed_in_robots as f32 / moved_field.robots.len() as f32;

            if boxed_in_percentage > boxed_in_threshold {
                println!(
                    "boxed in robots: {}/{} ({:.1}%) walk_counter: {}",
                    boxed_in_robots,
                    moved_field.robots.len(),
                    boxed_in_percentage * 100.0,
                    walk_counter
                );
                println!("{}", moved_field);

                return Some(walk_counter);
            }

            if *self == moved_field {
                break;
            }
        }

        None
    }
}

impl fmt::Display for RobotField {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let count = self
                    .robots
                    .iter()
                    .filter(|r| r.position == vector![x, y])
                    .count();
                if count == 0 {
                    write!(fmt, ".")?;
                } else {
                    write!(fmt, "{}", count)?;
                }
            }
            write!(fmt, "\n")?;
        }

        Ok(())
    }
}

static ROBOT_MOVEMENT_REGEX: &str = r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)";
fn parse_input(input: &str) -> RobotField {
    let re = Regex::new(ROBOT_MOVEMENT_REGEX).unwrap();

    let movements: Vec<RobotMovement> = re
        .captures_iter(input.trim())
        .map(|c| c.extract())
        .map(|(_, [p_x, p_y, v_x, v_y])| RobotMovement {
            position: vector![p_x.parse::<i32>().unwrap(), p_y.parse::<i32>().unwrap()],
            velocity: vector![v_x.parse::<i32>().unwrap(), v_y.parse::<i32>().unwrap()],
        })
        .collect();

    let max_x = movements.iter().map(|m| m.position.x).max().unwrap() + 1;
    let max_y = movements.iter().map(|m| m.position.y).max().unwrap() + 1;

    RobotField {
        robots: movements,
        size: vector![max_x, max_y],
    }
}

impl Solution for Day14 {
    fn solve_part1(&self, input: &str) -> String {
        let field = parse_input(input);
        field.compute_safety_factor(100).to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let field = parse_input(input);

        field
            .find_christmas_tree(0.2)
            .unwrap_or_default()
            .to_string()
    }
}
