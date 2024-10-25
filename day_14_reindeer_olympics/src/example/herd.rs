use super::reindeer::Reindeer;

pub struct Herd {
    herd: Vec<Reindeer>,
}

impl Herd {
    pub fn new(herd: Vec<Reindeer>) -> Self {
        Self { herd }
    }

    pub fn race_winning_distance(&self, seconds: u32) -> u32 {
        self.herd
            .iter()
            .map(|r| r.distance(seconds))
            .max()
            .expect("Failed to find max")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_winning_distance() {
        let herd = Herd::new(vec![
            Reindeer::new("Comet", 14, 10, 127),
            Reindeer::new("Dancer", 16, 11, 162),
        ]);

        assert_eq!(herd.race_winning_distance(1), 16);
        assert_eq!(herd.race_winning_distance(10), 160);
        assert_eq!(herd.race_winning_distance(12), 176);
        assert_eq!(herd.race_winning_distance(174), 280);
        assert_eq!(herd.race_winning_distance(1000), 1120);
    }
}
