use super::instruction::Instruction;

#[derive(Debug)]
pub struct Instructions {
    pub data: Vec<Instruction>,
}

impl Instructions {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self { data: instructions }
    }
}
