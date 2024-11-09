use crate::example::stack_pointer::StackPointerOffset;

use super::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct Jump {
    offset: i32,
}

impl Jump {
    pub fn new(offset: i32) -> Self {
        Self { offset }
    }

    pub fn get_offset(&self) -> StackPointerOffset {
        self.offset
    }
}

impl Instruction for Jump {
    fn execute(
        &self,
        _registers: &mut crate::example::registers::Registers,
    ) -> Result<StackPointerOffset, crate::example::cpu_error::CpuError> {
        Ok(self.offset)
    }
}
