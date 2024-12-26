use crate::solution::Solution;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::str::FromStr;
use z3::ast::{Ast, Bool, Dynamic, Int};
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
    fn get_all_wrires(&self) -> Vec<String> {
        self.connections
            .iter()
            .flat_map(|c| vec![c.left.clone(), c.right.clone(), c.output.clone()])
            .collect::<Vec<_>>()
    }

    fn find_swap_var<'a>(
        a: &String,
        b: &String,
        map: &'a HashMap<(String, String), Bool>,
    ) -> &'a Bool<'a> {
        let (&left, &right) = vec![a, b].iter().sorted().collect_tuple().unwrap();
        &map.get(&(left.to_string(), right.to_string())).unwrap()
    }

    fn get_number_from_bits<'a>(
        letter: char,
        context: &'a Context,
        wire_bools: &'a HashMap<String, Bool<'a>>,
    ) -> Int<'a> {
        let letter_wires = wire_bools
            .keys()
            .filter(|k| k.starts_with(letter))
            .sorted()
            .map(|k| wire_bools.get(k).unwrap())
            .collect::<Vec<_>>();

        let mut bitvec = ast::BV::from_u64(&context, 0, 1);
        for (i, b) in letter_wires.iter().rev().enumerate() {
            // Turn each Bool into a 1-bit BV: true => 1, else => 0
            let bit = b.ite(
                &ast::BV::from_u64(&context, 1, 1),
                &ast::BV::from_u64(&context, 0, 1),
            );
            if i == 0 {
                bitvec = bit;
            } else {
                bitvec = bitvec.concat(&bit);
            }
        }

        bitvec.to_int(false)
    }

    fn collect_variables(ast: &Dynamic, variables: &mut HashSet<String>) {
        // If the AST node is a variable, add its name
        if ast.is_const() {
            variables.insert(ast.decl().name().to_string());
        }

        // Recursively process child AST nodes
        for child in ast.children() {
            Self::collect_variables(&child, variables);
        }
    }

    fn solve_swap(&self) -> String {
        let all_wires = self.get_all_wrires();

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

        // create swap vars
        let output_vars = self
            .connections
            .iter()
            .map(|c| c.output.clone())
            .sorted()
            .collect_vec();

        let mut swap_vars: HashMap<(String, String), Bool> = HashMap::new();
        for i in 0..output_vars.len() {
            for j in i..output_vars.len() {
                let left = &output_vars[i];
                let right = &output_vars[j];
                let swap_var = Bool::new_const(&ctx, format!("swap_{left}_{right}"));
                swap_vars.insert((left.to_string(), right.to_string()), swap_var);
            }
        }

        // Create gates
        for gate in &self.connections {
            let left = wire_bools.get(&gate.left).unwrap();
            let right = wire_bools.get(&gate.right).unwrap();

            let current_swap_vars = output_vars
                .iter()
                .map(|o| Self::find_swap_var(&gate.output, o, &swap_vars))
                .collect_vec();

            // Only one swap var can be set, so sum over all == 1
            let pbeq_input: Vec<(&Bool, i32)> = current_swap_vars.iter().map(|&v| (v, 1)).collect();
            solver.assert(&Bool::pb_eq(&ctx, &pbeq_input, 1));

            let choice_parts = output_vars
                .iter()
                .map(|o| {
                    (
                        wire_bools.get(&*o).unwrap(),
                        Self::find_swap_var(&gate.output, o, &swap_vars),
                    )
                })
                .map(|(a, b)| Bool::and(&ctx, &[a, b]))
                .collect_vec();

            let choice_expr = choice_parts
                .iter()
                .skip(1)
                .fold(choice_parts[0].clone(), |acc, item| {
                    Bool::or(&ctx, &[&acc, item])
                });

            match gate.operation {
                GateType::AND => {
                    solver.assert(&choice_expr._eq(&Bool::and(&ctx, &[left, right])));
                }
                GateType::XOR => {
                    solver.assert(&choice_expr._eq(&left.xor(&right)));
                }
                GateType::OR => {
                    solver.assert(&choice_expr._eq(&Bool::or(&ctx, &[left, right])));
                }
            }
        }

        let z = Self::get_number_from_bits('z', &ctx, &wire_bools);
        let x = Self::get_number_from_bits('x', &ctx, &wire_bools);
        let y = Self::get_number_from_bits('y', &ctx, &wire_bools);

        solver.assert(&z._eq(&x.add(&y)));

        // number of actual swaps == 4
        let non_self_swaps = swap_vars
            .iter()
            .filter(|((l, r), _)| l != r)
            .map(|(_, v)| v)
            .collect_vec();
        let pbeq_input: Vec<(&Bool, i32)> = non_self_swaps.iter().map(|&v| (v, 1)).collect();

        solver.assert(&Bool::pb_eq(&ctx, &pbeq_input, 4));

        let mut variable_names = HashSet::new();
        for assertion in solver.get_assertions() {
            let assertion_dyn = Dynamic::from_ast(&assertion);
            Self::collect_variables(&assertion_dyn, &mut variable_names);
        }

        println!(
            "Model assertions: {}, vars: {}",
            solver.get_assertions().len(),
            variable_names.len()
        );

        println!("solving..");

        if solver.check() == z3::SatResult::Sat {
            println!("solved! :)");
            let model = solver.get_model().unwrap();
            let mut swapped = vec![];
            for ((left, right), swap_bool) in swap_vars {
                let swap_bool_model = model.eval(&swap_bool, true).unwrap().as_bool().unwrap();
                if left != right && swap_bool_model {
                    println!("{} <--> {}", left, right);
                    swapped.extend(vec![left, right]);
                }
            }
            swapped.sort();

            swapped.join(",")
        } else {
            "Unsat".to_string()
        }
    }

    fn solve_z(&self) -> String {
        let all_wires = self.get_all_wrires();

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
        let output = Self::get_number_from_bits('z', &ctx, &wire_bools);

        let mut variable_names = HashSet::new();
        for assertion in solver.get_assertions() {
            let assertion_dyn = Dynamic::from_ast(&assertion);
            Self::collect_variables(&assertion_dyn, &mut variable_names);
        }

        println!(
            "Model assertions: {}, vars: {}",
            solver.get_assertions().len(),
            variable_names.len()
        );

        if solver.check() == z3::SatResult::Sat {
            let model = solver.get_model().unwrap();
            let output_result = model.eval(&output, true).unwrap();
            // for (name, wire_bool) in wire_bools {
            //     let test = model.eval(&wire_bool, true).unwrap();
            //     println!("{} = {}", name, test.as_bool().unwrap());
            // }
            format!("Solved number: {}", output_result.as_u64().unwrap())
        } else {
            "Unsat".to_string()
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
        .map(|(_, [operand1, gate, operand2, output])| Gate {
            left: operand1.into(),
            right: operand2.into(),
            operation: GateType::from_str(gate).unwrap(),
            output: output.into(),
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

        wiring.solve_z()
    }

    fn solve_part2(&self, input: &str) -> String {
        let wiring = parse_input(input);

        wiring.solve_swap()
    }
}
