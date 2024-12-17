use crate::day17::computer::{Computer, Registers};

static COMBO_LITERAL_OPERAND_LIMIT: i64 = 3;

pub trait Operation {
    fn execute(&self, operand: i64, computer: &mut Computer) {
        self.execute_operation(self.get_operand(operand, &computer.registers), computer);
    }

    fn execute_operation(&self, operand: i64, computer: &mut Computer);

    fn operand_is_combo(&self) -> bool {
        true
    }

    fn get_operand(&self, operand: i64, registers: &Registers) -> i64 {
        if operand <= COMBO_LITERAL_OPERAND_LIMIT {
            return operand;
        }

        if self.operand_is_combo() {
            match operand {
                4 => registers.a,
                5 => registers.b,
                6 => registers.c,
                _ => 0,
            }
        } else {
            operand
        }
    }
}

pub struct OperationADV;
pub struct OperationBXL;
pub struct OperationBST;
pub struct OperationJNZ;
pub struct OperationBXC;
pub struct OperationOUT;
pub struct OperationBDV;
pub struct OperationCDV;

fn divide_by_power_of(num: i64, denum: i64) -> i64 {
    num / 2_i64.pow(denum as u32)
}

impl Operation for OperationADV {
    fn execute_operation(&self, operand: i64, computer: &mut Computer) {
        computer.registers.a = divide_by_power_of(computer.registers.a, operand);
    }
}

impl Operation for OperationBXL {
    fn execute_operation(&self, operand: i64, computer: &mut Computer) {
        computer.registers.b ^= operand;
    }

    fn operand_is_combo(&self) -> bool {
        false
    }
}

impl Operation for OperationBST {
    fn execute_operation(&self, operand: i64, computer: &mut Computer) {
        computer.registers.b = operand % 8;
    }
}

impl Operation for OperationJNZ {
    fn execute_operation(&self, operand: i64, computer: &mut Computer) {
        if computer.registers.a != 0 {
            computer.instruction_pointer = (operand / 2) as usize;
        }
    }

    fn operand_is_combo(&self) -> bool {
        false
    }
}

impl Operation for OperationBXC {
    fn execute_operation(&self, _operand: i64, computer: &mut Computer) {
        computer.registers.b ^= computer.registers.c;
    }
}

impl Operation for OperationOUT {
    fn execute_operation(&self, operand: i64, computer: &mut Computer) {
        computer.output.push(operand % 8);
    }
}

impl Operation for OperationBDV {
    fn execute_operation(&self, operand: i64, computer: &mut Computer) {
        computer.registers.b = divide_by_power_of(computer.registers.a, operand);
    }
}

impl Operation for OperationCDV {
    fn execute_operation(&self, operand: i64, computer: &mut Computer) {
        computer.registers.c = divide_by_power_of(computer.registers.a, operand);
    }
}
