use regex::Regex;
use crate::day17::computer::Registers;
use crate::day17::instruction::Instruction;
use crate::day17::operation::*;

pub struct ComputerParser;

impl ComputerParser {
    fn build_operation(opcode: i64) -> Box<dyn Operation> {
        match opcode {
            0 => Box::new(OperationADV),
            1 => Box::new(OperationBXL),
            2 => Box::new(OperationBST),
            3 => Box::new(OperationJNZ),
            4 => Box::new(OperationBXC),
            5 => Box::new(OperationOUT),
            6 => Box::new(OperationBDV),
            7 => Box::new(OperationCDV),
            _ => panic!("Invalid operation code: {}", opcode),
        }
    }

    pub(crate) fn parse(input: &str) -> (Registers, Vec<Instruction>, Vec<i64>) {
        let register_re = Regex::new(r"Register (\w):\s*(-?\d+)").unwrap();
        let program_re = Regex::new(r"Program:\s*(.*)").unwrap();

        let mut registers = Registers { a: 0, b: 0, c: 0 };
        let mut instructions = Vec::new();
        let mut raw_instructions = Vec::new();

        for line in input.lines() {
            let line = line.trim();

            // Match register lines
            if let Some(caps) = register_re.captures(line) {
                let register = &caps[1];
                let value = caps[2].parse::<i64>().unwrap_or(0);

                match register {
                    "A" => registers.a = value,
                    "B" => registers.b = value,
                    "C" => registers.c = value,
                    _ => {}
                }
            }

            // Match program line and parse pairs
            if let Some(caps) = program_re.captures(line) {
                let program_values: Vec<i64> = caps[1]
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();

                // Parse values as pairs: (operation, operand)
                for pair in program_values.chunks(2) {
                    instructions.push(Instruction {
                        operation: Self::build_operation(pair[0]),
                        operand: pair[1],
                    });
                    
                    raw_instructions.push(pair[0]);
                    raw_instructions.push(pair[1]);
                }
            }
        }

        (registers, instructions, raw_instructions)
    }
}