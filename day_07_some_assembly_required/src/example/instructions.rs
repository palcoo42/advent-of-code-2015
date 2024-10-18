use super::instruction::Instruction;

#[derive(Default)]
pub struct Instructions {
    pub data: Vec<Instruction>,
}

impl Instructions {
    pub fn new(data: Vec<Instruction>) -> Self {
        Self { data }
    }
}
