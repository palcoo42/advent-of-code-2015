use crate::example::{
    cpu_error::CpuError, registers::Registers, stack_pointer::StackPointerOffset,
};

use super::instruction::Instruction;

const NEXT_INSTRUCTION_OFFSET: StackPointerOffset = 1;

#[derive(Debug, Clone)]
pub struct Triple {
    reg_name: String,
}

impl Triple {
    pub fn new(register: &str) -> Self {
        Self {
            reg_name: register.to_string(),
        }
    }

    pub fn get_register_name(&self) -> &str {
        &self.reg_name
    }
}

impl Instruction for Triple {
    fn execute(&self, registers: &mut Registers) -> Result<StackPointerOffset, CpuError> {
        let reg_value = registers.get(&self.reg_name).ok_or_else(|| {
            CpuError::InstructionError(format!(
                "Failed to read value of register '{}'",
                self.reg_name
            ))
        })?;

        registers.set(&self.reg_name, *reg_value * 3);
        Ok(NEXT_INSTRUCTION_OFFSET)
    }
}
