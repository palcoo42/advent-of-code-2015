use super::bulb::Bulb;

#[derive(Default)]
pub struct BulbComplex {
    brightness: usize,
}

impl Bulb for BulbComplex {
    fn turn_on(&mut self) {
        self.brightness += 1;
    }

    fn turn_off(&mut self) {
        if self.brightness > 0 {
            self.brightness -= 1;
        }
    }

    fn toggle(&mut self) {
        self.brightness += 2;
    }

    fn is_on(&self) -> bool {
        self.brightness > 0
    }

    fn brightness(&self) -> usize {
        self.brightness
    }
}
