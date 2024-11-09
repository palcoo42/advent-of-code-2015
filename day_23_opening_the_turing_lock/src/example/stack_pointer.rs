use super::cpu_error::CpuError;

pub type StackPointerOffset = i32;

pub struct StackPointer {
    min: usize,
    max: usize,
    current: usize,
}

impl StackPointer {
    pub fn new(max: usize) -> Self {
        Self {
            min: 0,
            max,
            current: 0,
        }
    }

    pub fn get(&self) -> usize {
        self.current
    }

    pub fn reset(&mut self) {
        self.current = 0;
    }

    pub fn update(&mut self, offset: StackPointerOffset) -> Result<(), CpuError> {
        // Calculate new offset
        let new = self.current as i32 + offset;

        // Check offset validity
        if new < self.min as i32 || new > self.max as i32 {
            return Err(CpuError::StackPointerError(format!(
                "Failed to update stack pointer '{} <{};{}>' with offset '{}' [out of bounds]",
                self.current, self.min, self.max, offset
            )));
        }

        self.current = new as usize;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        const MIN: usize = 0;
        const MAX: usize = 5;

        let mut ptr = StackPointer::new(MAX);

        // Default value is pointing to the MIN
        assert_eq!(ptr.get(), MIN);

        // Move upwards
        assert!(ptr.update(1).is_ok());
        assert_eq!(ptr.get(), 1);

        assert!(ptr.update(2).is_ok());
        assert_eq!(ptr.get(), 3);

        assert!(ptr.update(2).is_ok());
        assert_eq!(ptr.get(), MAX);

        assert!(ptr.update(1).is_err());

        // Move downwards
        assert_eq!(ptr.get(), MAX);
        assert!(ptr.update(-1).is_ok());
        assert_eq!(ptr.get(), 4);

        assert!(ptr.update(-2).is_ok());
        assert_eq!(ptr.get(), 2);

        assert!(ptr.update(-2).is_ok());
        assert_eq!(ptr.get(), 0);

        assert!(ptr.update(-1).is_err());
    }
}
