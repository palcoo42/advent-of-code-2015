#[derive(Debug)]
pub struct Reindeer {
    pub speed: usize,
    pub active: usize,
    pub resting: usize,
}

impl Reindeer {
    pub fn distance(&self, seconds: usize) -> usize {
        let intervals = seconds / (self.active + self.resting);
        let remaining = seconds % (self.active + self.resting);

        (intervals * self.active + std::cmp::min(self.active, remaining)) * self.speed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let comet = Reindeer {
            speed: 14,
            active: 10,
            resting: 127,
        };
        let dancer = Reindeer {
            speed: 16,
            active: 11,
            resting: 162,
        };

        assert_eq!(comet.distance(1), 14);
        assert_eq!(dancer.distance(1), 16);

        assert_eq!(comet.distance(10), 140);
        assert_eq!(dancer.distance(10), 160);

        assert_eq!(comet.distance(11), 140);
        assert_eq!(dancer.distance(11), 176);

        assert_eq!(comet.distance(12), 140);
        assert_eq!(dancer.distance(12), 176);

        assert_eq!(comet.distance(137), 140);
        assert_eq!(dancer.distance(137), 176);

        assert_eq!(comet.distance(138), 154);
        assert_eq!(dancer.distance(138), 176);

        assert_eq!(dancer.distance(173), 176);
        assert_eq!(dancer.distance(174), 192);

        assert_eq!(comet.distance(1000), 1120);
        assert_eq!(dancer.distance(1000), 1056);
    }
}
