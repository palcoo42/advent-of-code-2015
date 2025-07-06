use crate::puzzle::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub instruction: Instruction,
    pub wire: String,
}
