use crate::example::stack_pointer::StackPointerOffset;

use super::{
    half::Half, increment::Increment, instruction::Instruction, jump::Jump,
    jump_if_even::JumpIfEven, jump_if_one::JumpIfOne, triple::Triple,
};

pub struct InstructionFactory {}

impl InstructionFactory {
    pub fn half(reg_name: &str) -> Box<dyn Instruction> {
        Box::new(Half::new(reg_name))
    }

    pub fn increment(reg_name: &str) -> Box<dyn Instruction> {
        Box::new(Increment::new(reg_name))
    }

    pub fn jump_if_even(reg_name: &str, offset: StackPointerOffset) -> Box<dyn Instruction> {
        Box::new(JumpIfEven::new(reg_name, offset))
    }

    pub fn jump_if_one(reg_name: &str, offset: StackPointerOffset) -> Box<dyn Instruction> {
        Box::new(JumpIfOne::new(reg_name, offset))
    }

    pub fn jump(offset: StackPointerOffset) -> Box<dyn Instruction> {
        Box::new(Jump::new(offset))
    }

    pub fn triple(reg_name: &str) -> Box<dyn Instruction> {
        Box::new(Triple::new(reg_name))
    }
}
