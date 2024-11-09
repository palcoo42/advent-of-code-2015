use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum CpuError {
    RegisterError(String),
    InstructionError(String),
    StackPointerError(String),
    GenericError(String),
}

impl Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info = match self {
            CpuError::RegisterError(msg) => format!("Register Error: {}", msg),
            CpuError::InstructionError(msg) => format!("Instruction Error: {}", msg),
            CpuError::StackPointerError(msg) => format!("Stack Pointer Error: {}", msg),
            CpuError::GenericError(msg) => format!("Generic Error: {}", msg),
        };
        write!(f, "{}", info)
    }
}

impl Error for CpuError {}
