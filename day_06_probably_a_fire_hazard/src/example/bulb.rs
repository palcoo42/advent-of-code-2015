#[derive(Debug, PartialEq)]
pub enum Bulb {
    On,
    Off,
}

impl Bulb {
    pub fn toggle(&mut self) {
        *self = match self {
            Bulb::On => Bulb::Off,
            Bulb::Off => Bulb::On,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle() {
        let mut bulb = Bulb::Off;

        assert_eq!(bulb, Bulb::Off);

        bulb.toggle();
        assert_eq!(bulb, Bulb::On);

        bulb.toggle();
        assert_eq!(bulb, Bulb::Off);
    }
}
