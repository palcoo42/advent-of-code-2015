use std::fmt::Debug;

use crate::example::{
    cpu_error::CpuError, registers::Registers, stack_pointer::StackPointerOffset,
};

pub trait Instruction: Debug {
    // Execute the instruction
    fn execute(&self, registers: &mut Registers) -> Result<StackPointerOffset, CpuError>;
}
