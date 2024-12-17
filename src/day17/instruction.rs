use crate::day17::computer::Computer;
use crate::day17::operation::{*};

pub struct Instruction {
    pub operation: Box<dyn Operation>,
    pub operand: i64,
}

impl Instruction {
    pub fn execute(&self, computer: &mut Computer) {
        self.operation.execute(self.operand, computer);
    }
}