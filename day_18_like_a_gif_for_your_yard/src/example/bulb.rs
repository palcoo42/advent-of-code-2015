use super::light::Light;

#[derive(Debug, PartialEq)]
pub struct Bulb {
    light: Light,
    neighbors_on: u32,
    stuck: bool, // Part 2
}

impl Bulb {
    pub fn new(light: Light) -> Self {
        Self {
            light,
            neighbors_on: 0,
            stuck: false,
        }
    }

    pub fn light(&self) -> &Light {
        &self.light
    }

    pub fn neighbors_on(&self) -> u32 {
        self.neighbors_on
    }

    pub fn set_light(&mut self, light: Light) {
        // Ignore changing of the light state when bulb is 'stuck', i.e. always Light::On
        if !self.stuck {
            self.light = light
        }
    }

    pub fn set_neighbors_on(&mut self, count: u32) {
        self.neighbors_on = count;
    }

    pub fn set_stuck(&mut self) {
        self.stuck = true;
        self.light = Light::On;
    }
}
