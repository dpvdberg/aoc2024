use crate::solution::Solution;
use maplit::hashmap;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[cfg(test)]
mod test;
pub struct Day7 {}

struct Equation {
    target: u64,
    operands: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (target, operands) = l.split_once(':').unwrap();
            Equation {
                target: target.trim().parse::<u64>().unwrap(),
                operands: operands
                    .trim()
                    .split(' ')
                    .map(|x| x.trim().parse::<u64>().unwrap())
                    .collect(),
            }
        })
        .collect()
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

static OPERATIONS: Lazy<HashMap<Operation, fn(&u64, &u64) -> u64>> = Lazy::new(|| {
    hashmap! {
        Operation::Add => (|a: &u64, b: &u64| a + b) as fn(&u64, &u64) -> u64,
        Operation::Multiply => (|a: &u64, b: &u64| a * b) as fn(&u64, &u64) -> u64,
        Operation::Concatenate => (|a: &u64, b: &u64| a * 10u64.pow(b.ilog10() + 1) + b) as fn(&u64, &u64) -> u64,
    }
});

fn check_equation_solvable(equation: &Equation, allow_concatenate: bool) -> bool {
    if equation.operands.len() == 1 {
        return equation.operands[0] == equation.target;
    }

    let [first, second, rest @ ..] = &equation.operands[..] else {
        panic!("Could not destruct operands")
    };

    if first > &equation.target {
        // Early exit
        return false;
    }

    for operation_type in OPERATIONS.keys() {
        if *operation_type == Operation::Concatenate && !allow_concatenate {
            continue;
        }

        let operation = OPERATIONS.get(operation_type).unwrap();
        let new_operand = operation(first, second);

        let mut new_operands = vec![new_operand];
        new_operands.extend(rest);
        let new_eq = Equation {
            target: equation.target,
            operands: new_operands,
        };
        if check_equation_solvable(&new_eq, allow_concatenate) {
            return true;
        }
    }

    false
}

impl Solution for Day7 {
    fn solve_part1(input: &str) -> String {
        let equations = parse_input(input);
        equations
            .into_iter()
            .filter(|eq| check_equation_solvable(&eq, false))
            .map(|eq| eq.target)
            .sum::<u64>()
            .to_string()
    }

    fn solve_part2(input: &str) -> String {
        let equations = parse_input(input);
        equations
            .into_iter()
            .filter(|eq| check_equation_solvable(&eq, true))
            .map(|eq| eq.target)
            .sum::<u64>()
            .to_string()
    }
}
