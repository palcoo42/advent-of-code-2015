use std::cmp::min;

pub type FlySpeed = u32;
pub type FlyDuration = u32;
pub type RestDuration = u32;

#[derive(Debug, PartialEq)]
pub struct Reindeer {
    name: String,
    fly_speed: FlySpeed,
    fly_duration: FlyDuration,
    rest_duration: RestDuration,
    points: u32,
}

impl Reindeer {
    pub fn new(
        name: &str,
        fly_speed: FlySpeed,
        fly_duration: FlyDuration,
        rest_duration: RestDuration,
    ) -> Self {
        Self {
            name: name.to_string(),
            fly_speed,
            fly_duration,
            rest_duration,
            points: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fly_speed(&self) -> FlySpeed {
        self.fly_speed
    }

    pub fn fly_duration(&self) -> FlyDuration {
        self.fly_duration
    }

    pub fn rest_duration(&self) -> RestDuration {
        self.rest_duration
    }

    pub fn distance(&self, seconds: u32) -> u32 {
        let single_round = self.fly_duration + self.rest_duration;
        let rounds = seconds / single_round;
        let delta = seconds % single_round;

        // Final distance = number of rounds completed multiplied by distance of a single round +
        //                  distance traveled in last round
        rounds * self.fly_duration * self.fly_speed + min(delta, self.fly_duration) * self.fly_speed
    }

    pub fn reset_points(&mut self) {
        self.points = 0;
    }

    pub fn points(&self) -> u32 {
        self.points
    }

    pub fn award_point(&mut self) {
        self.points += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let comet = Reindeer::new("Comet", 14, 10, 127);
        let dancer = Reindeer::new("Dancer", 16, 11, 162);

        assert_eq!(comet.distance(1), 14);
        assert_eq!(comet.distance(10), 140);
        assert_eq!(comet.distance(11), 140);
        assert_eq!(comet.distance(12), 140);
        assert_eq!(comet.distance(138), 154);
        assert_eq!(comet.distance(139), 168);
        assert_eq!(comet.distance(174), 280);
        assert_eq!(comet.distance(1000), 1120);

        assert_eq!(dancer.distance(1), 16);
        assert_eq!(dancer.distance(10), 160);
        assert_eq!(dancer.distance(11), 176);
        assert_eq!(dancer.distance(12), 176);
        assert_eq!(dancer.distance(138), 176);
        assert_eq!(dancer.distance(139), 176);
        assert_eq!(dancer.distance(174), 192);
        assert_eq!(dancer.distance(1000), 1056);
    }
}
