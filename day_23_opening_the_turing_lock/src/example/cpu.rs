use super::{
    cpu_error::CpuError, instructions::Instructions, registers::Registers,
    stack_pointer::StackPointer,
};

pub struct Cpu {
    instructions: Instructions,
    stack_pointer: StackPointer,
    registers: Registers,
}

impl Cpu {
    pub fn new(instructions: Instructions) -> Self {
        let stack_pointer = StackPointer::new(instructions.len());

        let mut registers = Registers::new();
        registers.set("a", 0);
        registers.set("b", 0);

        Self {
            instructions,
            stack_pointer,
            registers,
        }
    }

    pub fn run(&mut self) -> Result<(), CpuError> {
        // Reset stack pointer to start from the scratch
        self.stack_pointer.reset();

        // Execute the instructions
        loop {
            // Check for the last instruction
            if self.stack_pointer.get() >= self.instructions.len() {
                break;
            }

            // Execute the instruction
            let instr = self.instructions.get(&self.stack_pointer);
            if let Some(instruction) = instr {
                // Execute
                let offset = instruction.execute(&mut self.registers)?;

                // println!("a: {}", self.registers.get("a").unwrap());
                // println!("b: {}", self.registers.get("b").unwrap());
                // println!("ip: {}", self.stack_pointer.get());

                // Update stack pointer
                self.stack_pointer.update(offset)?;
            } else {
                return Err(CpuError::StackPointerError(format!(
                    "Invalid stack pointer index '{}', instructions: {:?}",
                    self.stack_pointer.get(),
                    self.instructions
                )));
            }
        }

        Ok(())
    }

    pub fn get_register(&self, register: &str) -> Option<&u32> {
        self.registers.get(register)
    }
}

#[cfg(test)]
mod tests {
    use crate::example::cpu_instructions::{
        half::Half, increment::Increment, instruction::Instruction,
        instruction_factory::InstructionFactory, jump::Jump, jump_if_even::JumpIfEven,
        jump_if_one::JumpIfOne, triple::Triple,
    };

    use super::*;

    #[test]
    fn test_run_increment() {
        let instructions: Vec<Box<dyn Instruction>> = vec![
            Box::new(Increment::new("a")),
            Box::new(Increment::new("b")),
            Box::new(Increment::new("a")),
            Box::new(Increment::new("a")),
            Box::new(Increment::new("b")),
        ];
        let mut cpu = Cpu::new(Instructions::new_from_raw(instructions));

        assert_eq!(cpu.get_register("a"), Some(&0));
        assert_eq!(cpu.get_register("b"), Some(&0));

        let result = cpu.run();

        assert!(result.is_ok(), "Failed with error: {:?}", result);
        assert_eq!(cpu.get_register("a"), Some(&3));
        assert_eq!(cpu.get_register("b"), Some(&2));
    }

    #[test]
    fn test_run_half() {
        let instructions: Vec<Box<dyn Instruction>> = vec![
            Box::new(Half::new("a")),
            Box::new(Increment::new("b")),
            Box::new(Increment::new("a")),
            Box::new(Increment::new("a")),
            Box::new(Half::new("a")),
            Box::new(Half::new("b")),
        ];
        let mut cpu = Cpu::new(Instructions::new_from_raw(instructions));

        assert_eq!(cpu.get_register("a"), Some(&0));
        assert_eq!(cpu.get_register("b"), Some(&0));

        let result = cpu.run();

        assert!(result.is_ok(), "Failed with error: {:?}", result);
        assert_eq!(cpu.get_register("a"), Some(&1));
        assert_eq!(cpu.get_register("b"), Some(&0));
    }

    #[test]
    fn test_run_triple() {
        let instructions: Vec<Box<dyn Instruction>> = vec![
            Box::new(Triple::new("a")),
            Box::new(Increment::new("b")),
            Box::new(Increment::new("a")),
            Box::new(Increment::new("a")),
            Box::new(Triple::new("a")),
            Box::new(Triple::new("b")),
        ];
        let mut cpu = Cpu::new(Instructions::new_from_raw(instructions));

        assert_eq!(cpu.get_register("a"), Some(&0));
        assert_eq!(cpu.get_register("b"), Some(&0));

        let result = cpu.run();

        assert!(result.is_ok(), "Failed with error: {:?}", result);
        assert_eq!(cpu.get_register("a"), Some(&6));
        assert_eq!(cpu.get_register("b"), Some(&3));
    }

    #[test]
    fn test_run_jump() {
        let instructions: Vec<Box<dyn Instruction>> = vec![
            Box::new(Increment::new("a")),
            Box::new(Jump::new(1)),
            Box::new(Increment::new("a")),
            Box::new(Jump::new(6)),
            Box::new(Increment::new("a")),
            Box::new(Increment::new("a")),
            Box::new(Increment::new("a")),
            Box::new(Increment::new("a")),
            Box::new(Jump::new(2)),
            Box::new(Jump::new(-3)),
        ];
        let mut cpu = Cpu::new(Instructions::new_from_raw(instructions));

        assert_eq!(cpu.get_register("a"), Some(&0));
        assert_eq!(cpu.get_register("b"), Some(&0));

        let result = cpu.run();

        assert!(result.is_ok(), "Failed with error: {:?}", result);
        assert_eq!(cpu.get_register("a"), Some(&4));
        assert_eq!(cpu.get_register("b"), Some(&0));
    }

    #[test]
    fn test_run_jump_if_even_jump() {
        let instructions: Vec<Box<dyn Instruction>> = vec![
            Box::new(JumpIfEven::new("a", 3)),
            Box::new(Increment::new("a")),
            Box::new(Increment::new("a")),
            Box::new(Increment::new("a")),
        ];
        let mut cpu = Cpu::new(Instructions::new_from_raw(instructions));

        assert_eq!(cpu.get_register("a"), Some(&0));
        assert_eq!(cpu.get_register("b"), Some(&0));

        let result = cpu.run();

        assert!(result.is_ok(), "Failed with error: {:?}", result);
        assert_eq!(cpu.get_register("a"), Some(&1));
        assert_eq!(cpu.get_register("b"), Some(&0));
    }

    #[test]
    fn test_run_jump_if_even_do_not_jump() {
        let instructions: Vec<Box<dyn Instruction>> = vec![
            Box::new(Increment::new("a")),
            Box::new(JumpIfEven::new("a", 3)),
            Box::new(Increment::new("a")),
            Box::new(Increment::new("a")),
            Box::new(Increment::new("a")),
        ];
        let mut cpu = Cpu::new(Instructions::new_from_raw(instructions));

        assert_eq!(cpu.get_register("a"), Some(&0));
        assert_eq!(cpu.get_register("b"), Some(&0));

        let result = cpu.run();

        assert!(result.is_ok(), "Failed with error: {:?}", result);
        assert_eq!(cpu.get_register("a"), Some(&4));
        assert_eq!(cpu.get_register("b"), Some(&0));
    }

    #[test]
    fn test_run_jump_if_one_jump() {
        let instructions: Vec<Box<dyn Instruction>> = vec![
            Box::new(Increment::new("a")),
            Box::new(JumpIfOne::new("a", 3)),
            Box::new(Increment::new("a")),
            Box::new(Increment::new("a")),
            Box::new(Increment::new("a")),
        ];
        let mut cpu = Cpu::new(Instructions::new_from_raw(instructions));

        assert_eq!(cpu.get_register("a"), Some(&0));
        assert_eq!(cpu.get_register("b"), Some(&0));

        let result = cpu.run();

        assert!(result.is_ok(), "Failed with error: {:?}", result);
        assert_eq!(cpu.get_register("a"), Some(&2));
        assert_eq!(cpu.get_register("b"), Some(&0));
    }

    #[test]
    fn test_run_jump_if_one_do_not_jump() {
        let instructions: Vec<Box<dyn Instruction>> = vec![
            Box::new(JumpIfOne::new("a", 3)),
            Box::new(Increment::new("a")),
            Box::new(Increment::new("a")),
            Box::new(Increment::new("a")),
        ];
        let mut cpu = Cpu::new(Instructions::new_from_raw(instructions));

        assert_eq!(cpu.get_register("a"), Some(&0));
        assert_eq!(cpu.get_register("b"), Some(&0));

        let result = cpu.run();

        assert!(result.is_ok(), "Failed with error: {:?}", result);
        assert_eq!(cpu.get_register("a"), Some(&3));
        assert_eq!(cpu.get_register("b"), Some(&0));
    }

    #[test]
    fn test_run() {
        // inc a
        // jio a, +2
        // tpl a
        // inc a
        let mut instructions = Instructions::new();
        instructions.push(InstructionFactory::increment("a"));
        instructions.push(InstructionFactory::jump_if_one("a", 2));
        instructions.push(InstructionFactory::triple("a"));
        instructions.push(InstructionFactory::increment("a"));

        let mut cpu = Cpu::new(instructions);
        let result = cpu.run();

        assert!(result.is_ok(), "Failed with error '{:?}'", result);
        assert_eq!(cpu.get_register("a"), Some(&2));
    }
}
