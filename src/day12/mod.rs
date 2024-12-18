use crate::solution::Solution;
use crate::utils::geometry::Direction;
use crate::utils::nalgebra::{MatrixParser, VectorHelpers};
use nalgebra::{DMatrix, Vector2};
use std::collections::{HashMap, HashSet, VecDeque};
use strum::IntoEnumIterator;

#[cfg(test)]
mod test;
pub struct Day12 {}

struct Plot {
    plant: char,
    positions: Vec<Vector2<i32>>,
    area: usize,
    perimeter: usize,
}

fn array_boundaries(values: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    if values.is_empty() {
        return (vec![], vec![]);
    }

    let mut v = values.clone();
    v.sort();

    let mut left = vec![];
    let mut right = vec![];

    for i in 0..v.len() {
        if i == 0 || v[i] > v[i - 1] + 1 {
            left.push(v[i] - 1);
        }
        if i == v.len() - 1 || v[i] + 1 < v[i + 1] {
            right.push(v[i] + 1);
        }
    }

    (left, right)
}

fn count_consecutive_values(values: &Vec<i32>) -> usize {
    array_boundaries(values).0.len()
}

impl Plot {
    fn count_sides(&self) -> usize {
        let y_min = self.positions.iter().map(|p| p.y).min().unwrap();
        let y_max = self.positions.iter().map(|p| p.y).max().unwrap();

        let mut left_slices: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut right_slices: HashMap<i32, Vec<i32>> = HashMap::new();

        for y in y_min..=y_max {
            let positions = self
                .positions
                .iter()
                .filter(|p| p.y == y)
                .collect::<Vec<_>>();
            let x_values = positions.iter().map(|p| p.x).collect::<Vec<_>>();
            let (left_fences, right_fences) = array_boundaries(&x_values);

            left_fences
                .iter()
                .for_each(|x| left_slices.entry(*x).or_insert_with(Vec::new).push(y));
            right_fences
                .iter()
                .for_each(|x| right_slices.entry(*x).or_insert_with(Vec::new).push(y));
        }

        let left_fences: usize = left_slices
            .values()
            .map(|p| count_consecutive_values(p))
            .sum();
        let right_fences: usize = right_slices
            .values()
            .map(|p| count_consecutive_values(p))
            .sum();

        // #(horizontal edges) equals #(vertical edges) in a rectilinear polygon
        2 * (right_fences + left_fences)
    }
}

struct Garden {
    plants: DMatrix<char>,
}

impl Garden {
    fn within_bounds(&self, l: &Vector2<i32>) -> bool {
        l.x >= 0 && l.x < self.plants.ncols() as i32 && l.y >= 0 && l.y < self.plants.nrows() as i32
    }

    fn get_neighbors(&self, location: &Vector2<i32>) -> Vec<Vector2<i32>> {
        Direction::iter()
            .map(|d| location + d.to_vector())
            .filter(|l| self.within_bounds(l))
            .collect::<Vec<Vector2<i32>>>()
    }

    fn explore_plot(
        &self,
        start: Vector2<i32>,
        visited: &mut HashSet<Vector2<i32>>,
    ) -> (Plot, Vec<Vector2<i32>>) {
        let mut current_plot_queue: VecDeque<Vector2<i32>> = VecDeque::new();
        let mut next_plot_starters: Vec<Vector2<i32>> = Vec::new();
        let mut plot = Plot {
            plant: self.plants[start.to_matrix_index()],
            positions: Vec::new(),
            area: 0,
            perimeter: 0,
        };

        current_plot_queue.push_back(start);

        while !current_plot_queue.is_empty() {
            let current = current_plot_queue.pop_front().unwrap();
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);
            plot.positions.push(current);

            let plant = self.plants[current.to_matrix_index()];
            let neighbors = self.get_neighbors(&current);

            let same_plot_neighbor_count = neighbors
                .iter()
                .filter(|n| self.plants[n.to_matrix_index()] == plant)
                .count();
            let added_perimeter = 4 - same_plot_neighbor_count;

            plot.perimeter += added_perimeter;
            plot.area += 1;

            for neighbor in self.get_neighbors(&current) {
                if visited.contains(&neighbor) {
                    continue;
                }

                let neighbor_plant = self.plants[neighbor.to_matrix_index()];
                if neighbor_plant == plant {
                    current_plot_queue.push_front(neighbor.clone());
                } else {
                    next_plot_starters.push(neighbor.clone());
                }
            }
        }

        (plot, next_plot_starters)
    }

    fn find_plots(&self) -> Vec<Plot> {
        let mut plot_starters: VecDeque<Vector2<i32>> = VecDeque::new();
        let mut visited: HashSet<Vector2<i32>> = HashSet::new();
        let mut plots: Vec<Plot> = Vec::new();

        let start = Vector2::new(0, 0);
        plot_starters.push_back(start);

        while !plot_starters.is_empty() {
            let current = plot_starters.pop_front().unwrap();
            if visited.contains(&current) {
                continue;
            }

            let (plot, next_plot_starters) = self.explore_plot(current, &mut visited);

            next_plot_starters
                .iter()
                .for_each(|p| plot_starters.push_back(*p));

            plots.push(plot);
        }

        plots
    }
}

fn parse_input(input: &str) -> Garden {
    let plants = input.to_string().to_matrix(|c| c);

    Garden { plants }
}

impl Solution for Day12 {
    fn solve_part1(&self, input: &str) -> String {
        let garden = parse_input(input);
        garden
            .find_plots()
            .iter()
            .map(|p| p.area * p.perimeter)
            .sum::<usize>()
            .to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let garden = parse_input(input);
        garden
            .find_plots()
            .iter()
            .map(|p| {
                println!("{}: {} * {}", p.plant, p.area, p.count_sides());

                p.area * p.count_sides()
            })
            .sum::<usize>()
            .to_string()
    }
}
