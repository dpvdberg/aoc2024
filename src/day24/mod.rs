use crate::solution::Solution;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use z3::ast::{Ast, Bool};
use z3::{ast, Config, Context, Solver};

#[cfg(test)]
mod test;
pub struct Day24 {}

#[derive(Debug)]
enum GateType {
    AND,
    XOR,
    OR,
}

impl FromStr for GateType {
    type Err = ();

    fn from_str(input: &str) -> Result<GateType, Self::Err> {
        match input {
            "AND" => Ok(GateType::AND),
            "XOR" => Ok(GateType::XOR),
            "OR" => Ok(GateType::OR),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Gate {
    left: String,
    right: String,
    operation: GateType,
    output: String,
}

#[derive(Debug)]
struct Wiring {
    initial_values: HashMap<String, bool>,
    connections: Vec<Gate>,
}

impl Wiring {
    fn solve_z(&self) -> u64 {
        let all_wires = self
            .connections
            .iter()
            .flat_map(|c| vec![c.left.clone(), c.right.clone(), c.output.clone()])
            .collect::<Vec<_>>();

        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

        let wire_bools = all_wires
            .iter()
            .map(|w| (w.clone(), Bool::new_const(&ctx, w.to_string())))
            .collect::<HashMap<_, _>>();

        // Set initial values
        for (wire, value) in &self.initial_values {
            let bool_var = wire_bools.get(&wire.clone()).unwrap();

            solver.assert(&bool_var._eq(&Bool::from_bool(&ctx, *value)));
        }

        // Create gates
        for gate in &self.connections {
            let left = wire_bools.get(&gate.left).unwrap();
            let right = wire_bools.get(&gate.right).unwrap();
            let output = wire_bools.get(&gate.output).unwrap();

            match gate.operation {
                GateType::AND => {
                    solver.assert(&output._eq(&Bool::and(&ctx, &[left, right])));
                }
                GateType::XOR => {
                    solver.assert(&output._eq(&left.xor(&right)));
                }
                GateType::OR => {
                    solver.assert(&output._eq(&Bool::or(&ctx, &[left, right])));
                }
            }
        }

        // Find output
        let z_wires = wire_bools
            .keys()
            .filter(|k| k.starts_with("z"))
            .sorted()
            .collect::<Vec<_>>();
        let z_vars = z_wires
            .iter()
            .map(|&k| wire_bools.get(k).unwrap())
            .collect::<Vec<_>>();

        // Convert z bools to bitvec
        let mut bitvec = ast::BV::from_u64(&ctx, 0, 1);
        for (i, b) in z_vars.iter().rev().enumerate() {
            // Turn each Bool into a 1-bit BV: true => 1, else => 0
            let bit = b.ite(
                &ast::BV::from_u64(&ctx, 1, 1),
                &ast::BV::from_u64(&ctx, 0, 1),
            );
            if i == 0 {
                bitvec = bit;
            } else {
                bitvec = bitvec.concat(&bit);
            }
        }

        let output = bitvec.to_int(false);

        if solver.check() == z3::SatResult::Sat {
            let model = solver.get_model().unwrap();
            let output_result = model.eval(&output, true).unwrap();
            // for (name, wire_bool) in wire_bools {
            //     let test = model.eval(&wire_bool, true).unwrap();
            //     println!("{} = {}", name, test.as_bool().unwrap());
            // }
            println!("Solved number: {}", output_result.as_u64().unwrap());

            output_result.as_u64().unwrap()
        } else {
            println!("Unsat");
            0
        }
    }
}

fn parse_input(input: &str) -> Wiring {
    let normalized = input.replace("\r\n", "\n");
    let lines: Vec<_> = normalized.lines().map(|l| l.trim()).collect();
    let processed_input = lines.join("\n");
    let (raw_initial_values, gates) = processed_input.split_once("\n\n").unwrap();

    let initial_values = raw_initial_values
        .trim()
        .lines()
        .map(|l| l.split_once(":").unwrap())
        .map(|(k, v)| (k.trim().to_string(), v.trim().parse::<u8>().unwrap()))
        .map(|(k, v)| (k, if v > 0 { true } else { false }))
        .collect::<HashMap<String, bool>>();

    let re = Regex::new(r"(.*?) (\w+) (.*?) -> (.*)").unwrap();

    let connections = re
        .captures_iter(gates.trim())
        .map(|c| c.extract())
        .map(|(_, [operand1, gate, operand2, output])| {
            Gate {
                left: operand1.into(),
                right: operand2.into(),
                operation: GateType::from_str(gate).unwrap(),
                output: output.into(),
            }
        })
        .collect::<Vec<Gate>>();

    Wiring {
        initial_values,
        connections,
    }
}

impl Solution for Day24 {
    fn solve_part1(&self, input: &str) -> String {
        let wiring = parse_input(input);

        wiring.solve_z().to_string()
    }

    fn solve_part2(&self, _input: &str) -> String {
        "".into()
    }
}
