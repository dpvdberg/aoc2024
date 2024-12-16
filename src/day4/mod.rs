use crate::solution::Solution;
use nalgebra::{DMatrix, Vector2};
use crate::utils::nalgebra::MatrixParser;

#[cfg(test)]
mod test;
pub struct Day4 {}

fn parse_input(input: &str) -> DMatrix<char> {
    input.to_string().to_matrix(|c| c)
}

fn get_directions(offsets_x: Vec<i32>, offsets_y: Vec<i32>) -> Vec<Vector2<i32>> {
    offsets_x
        .iter()
        .map(|&x| {
            offsets_y
                .iter()
                .map(|&y| Vector2::new(x, y))
                .collect::<Vec<Vector2<i32>>>()
        })
        .flatten()
        .collect()
}

fn count_occurrences(
    matrix: &DMatrix<char>,
    location: Vector2<i32>,
    directions: &Vec<Vector2<i32>>,
    needle: &str,
) -> u32 {

    if location.x < 0
        || location.y < 0
    {
        return 0;
    }

    let matrix_index = (location.y as usize, location.x as usize);
    if matrix.get(matrix_index).is_none()
    {
        return 0;
    }

    let current_char = *matrix.get(matrix_index).unwrap();
    let needed_char = needle.chars().next().unwrap();
    if needed_char != current_char {
        return 0;
    }

    if needle.chars().count() == 1 {
        return 1;
    }

    let mut sum = 0;
    for d in directions {
        let new_loc: Vector2<i32> = location + d;
        let fixed_direction = vec![d.clone()];

        let mut remaining_needle = needle.to_string();
        remaining_needle.remove(0);

        sum += count_occurrences(matrix, new_loc, &fixed_direction, remaining_needle.as_str());
    }

    sum
}

impl Solution for Day4 {
    fn solve_part1(input: &str) -> String {
        let matrix = parse_input(input);

        let directions = get_directions(
            vec![-1, 0, 1], vec![-1, 0, 1]);

        let mut sum: u32 = 0;
        for row in 0..matrix.nrows() {
            for col in 0..matrix.ncols() {
                let loc = Vector2::new(row as i32, col as i32);
                sum += count_occurrences(&matrix, loc, &directions, "XMAS");
            }
        }

        sum.to_string()
    }

    fn solve_part2(input: &str) -> String {
        let matrix = parse_input(input);
        let diagonals = get_directions(vec![-1, 1], vec![-1, 1]);
        let diagonal_groups = vec![
            get_directions(vec![1], vec![-1, 1]),
            get_directions(vec![-1], vec![-1, 1]),
            get_directions(vec![-1, 1], vec![1]),
            get_directions(vec![-1, 1], vec![-1]),
        ];

        let mut sum: u32 = 0;
        for row in 0..matrix.nrows() {
            for col in 0..matrix.ncols() {
                let loc = Vector2::new(row as i32, col as i32);
                for d in &diagonal_groups {
                    let count_s = count_occurrences(&matrix, loc, &d, "AS");
                    if count_s != 2 {
                        continue;
                    }
                    
                    let count_m = count_occurrences(&matrix, loc, &diagonals, "AM");
                    if count_m == 2 {
                        sum += 1;
                    }
                }
            }
        }

        sum.to_string()
    }
}
