use crate::day17::computer::Computer;
use crate::day17::parser::ComputerParser;
use crate::solution::Solution;

#[cfg(test)]
mod test;
mod operation;
mod computer;
mod instruction;
mod parser;

pub struct Day17 {}

impl Solution for Day17 {
    fn solve_part1(input: &str) -> String {
        let (registers, instructions, _) = ComputerParser::parse(input);
        let mut computer = Computer::new(registers);
        
        computer.execute(&instructions);
        
        computer.output.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")
    }

    fn solve_part2(input: &str) -> String {
        let (registers, instructions, raw_instructions) = ComputerParser::parse(input);
        
        // This works, but it is probably more elegant to do this using a SAT solver.
        
        let mut start_a = 0;
        
        for n in 1..=raw_instructions.len() {
            let target = raw_instructions.iter().rev().take(n).rev().copied().collect::<Vec<i64>>();
            
            let mut current_a = start_a << 3;
            
            loop {
                let mut new_registers = registers.clone();
                new_registers.a = current_a;
                
                let mut comp = Computer::new(new_registers);
                comp.execute(&instructions);

                if comp.output == target {
                    start_a = current_a;
                    break;
                }
                current_a += 1;
            }
        }
        
        start_a.to_string()
    }
}