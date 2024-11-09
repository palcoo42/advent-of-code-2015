use super::{cpu_instructions::instruction::Instruction, stack_pointer::StackPointer};

#[derive(Debug, Default)]
pub struct Instructions {
    internal: Vec<Box<dyn Instruction>>,
}

impl Instructions {
    pub fn new() -> Self {
        Self {
            internal: Vec::new(),
        }
    }

    pub fn new_from_raw(instructions: Vec<Box<dyn Instruction>>) -> Self {
        Self {
            internal: instructions,
        }
    }

    pub fn push(&mut self, instruction: Box<dyn Instruction>) {
        self.internal.push(instruction);
    }

    #[inline]
    pub fn get(&self, stack_pointer: &StackPointer) -> Option<&dyn Instruction> {
        self.internal.get(stack_pointer.get()).map(|instr| &**instr)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.internal.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.internal.is_empty()
    }
}
