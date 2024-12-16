use nalgebra::DMatrix;
use crate::solution::Solution;
use crate::utils::nalgebra::MatrixParser;

#[cfg(test)]
mod test;
pub struct Day16 {}

#[derive(Clone, PartialEq, Debug)]
enum MazeTile {
    Start,
    End,
    Wall,
    Floor,
}

fn parse_input(input: &str) -> DMatrix<MazeTile> {
    input.to_string().to_matrix(|c| match c {
        'S' => MazeTile::Start,
        'E' => MazeTile::End,
        '#' => MazeTile::Wall,
        '.' => MazeTile::Floor,
        _ => panic!("Unexpected character in input: {}", c),
    })
}

impl Solution for Day16 {
    fn solve_part1(_input: &str) -> String {
        
        "".into()
    }

    fn solve_part2(_input: &str) -> String {
        "".into()
    }
}
