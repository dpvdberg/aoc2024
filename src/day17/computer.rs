use crate::day17::instruction::Instruction;

#[derive(Clone)]
pub struct Registers {
    pub a: i64,
    pub b: i64,
    pub c: i64,
}

pub struct Computer {    
    pub registers: Registers,
    pub instruction_pointer: usize,
    pub output : Vec<i64>,
}

impl Computer {
    pub fn new(registers: Registers) -> Self {
        Self {
            registers,
            instruction_pointer: 0,
            output: vec![],
        }
    }
    
    pub fn execute(&mut self, instructions: &[Instruction]) {
        while self.instruction_pointer < instructions.len() {
            let pointer = self.instruction_pointer;
            let instruction = &instructions[pointer];
            instruction.execute(self);

            if pointer == self.instruction_pointer {
                self.instruction_pointer += 1;
            }
        }
    }
}
