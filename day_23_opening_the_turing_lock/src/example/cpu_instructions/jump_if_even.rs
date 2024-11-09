use crate::example::{
    cpu_error::CpuError, registers::Registers, stack_pointer::StackPointerOffset,
};

use super::instruction::Instruction;

const NEXT_INSTRUCTION_OFFSET: StackPointerOffset = 1;

#[derive(Debug, Clone)]
pub struct JumpIfEven {
    reg_name: String,
    offset: StackPointerOffset,
}

impl JumpIfEven {
    pub fn new(register: &str, offset: StackPointerOffset) -> Self {
        Self {
            reg_name: register.to_string(),
            offset,
        }
    }

    pub fn get_register_name(&self) -> &str {
        &self.reg_name
    }

    pub fn get_offset(&self) -> StackPointerOffset {
        self.offset
    }
}

impl Instruction for JumpIfEven {
    fn execute(&self, registers: &mut Registers) -> Result<StackPointerOffset, CpuError> {
        let reg_value = registers.get(&self.reg_name).ok_or_else(|| {
            CpuError::InstructionError(format!("Failed to read register '{}' value", self.reg_name))
        })?;

        let offset = match reg_value % 2 == 0 {
            true => self.offset,
            false => NEXT_INSTRUCTION_OFFSET,
        };

        Ok(offset)
    }
}
