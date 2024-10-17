use super::bulb::Bulb;

#[derive(Debug, Default, PartialEq)]
pub enum BulbSimple {
    On,
    #[default]
    Off,
}

impl Bulb for BulbSimple {
    fn turn_on(&mut self) {
        *self = BulbSimple::On
    }

    fn turn_off(&mut self) {
        *self = BulbSimple::Off
    }

    fn toggle(&mut self) {
        *self = match self {
            BulbSimple::On => BulbSimple::Off,
            BulbSimple::Off => BulbSimple::On,
        }
    }

    fn is_on(&self) -> bool {
        *self == BulbSimple::On
    }

    fn brightness(&self) -> usize {
        match self {
            BulbSimple::On => 1,
            BulbSimple::Off => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle() {
        let mut bulb = BulbSimple::Off;

        assert_eq!(bulb, BulbSimple::Off);

        bulb.toggle();
        assert_eq!(bulb, BulbSimple::On);

        bulb.toggle();
        assert_eq!(bulb, BulbSimple::Off);
    }
}
