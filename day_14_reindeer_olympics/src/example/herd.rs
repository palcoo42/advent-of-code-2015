use super::reindeer::Reindeer;

pub struct Herd {
    herd: Vec<Reindeer>,
}

impl Herd {
    pub fn new(herd: Vec<Reindeer>) -> Self {
        Self { herd }
    }

    pub fn race_winning_by_distance(&self, seconds: u32) -> u32 {
        self.herd
            .iter()
            .map(|r| r.distance(seconds))
            .max()
            .expect("Failed to find max")
    }

    pub fn race_winning_by_points(&mut self, seconds: u32) -> u32 {
        // Reset all points
        self.herd.iter_mut().for_each(|r| r.reset_points());

        // Start race
        for current_second in 1..=seconds {
            // Move all reindeers
            let current_distances = self
                .herd
                .iter()
                .map(|r| r.distance(current_second))
                .collect::<Vec<_>>();

            // Award points to the reindeers which are the furthest
            let max = *current_distances
                .iter()
                .max()
                .expect("Failed to find max in 'current_distances'");

            // Note: Items in self.herd match in order with current_distances
            if self.herd.len() != current_distances.len() {
                panic!("Mismatch of 'herd' and 'current_distances'");
            }

            let winners = self
                .herd
                .iter_mut()
                .zip(current_distances)
                .filter_map(|(reindeer, distance)| {
                    if distance == max {
                        Some(reindeer)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            for winner in winners {
                winner.award_point();
            }
        }

        // Find the winner -> with maximum points
        self.herd
            .iter()
            .map(|reindeer| reindeer.points())
            .max()
            .expect("Failed to find max")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_winning_by_distance() {
        let herd = Herd::new(vec![
            Reindeer::new("Comet", 14, 10, 127),
            Reindeer::new("Dancer", 16, 11, 162),
        ]);

        assert_eq!(herd.race_winning_by_distance(1), 16);
        assert_eq!(herd.race_winning_by_distance(10), 160);
        assert_eq!(herd.race_winning_by_distance(12), 176);
        assert_eq!(herd.race_winning_by_distance(174), 280);
        assert_eq!(herd.race_winning_by_distance(1000), 1120);
    }

    #[test]
    fn test_race_winning_by_points() {
        let mut herd = Herd::new(vec![
            Reindeer::new("Comet", 14, 10, 127),
            Reindeer::new("Dancer", 16, 11, 162),
        ]);
        // Helpers to index into 'herd' safely
        const COMET_INDEX: usize = 0;
        const DANCER_INDEX: usize = 1;

        assert_eq!(herd.race_winning_by_points(1), 1);
        assert_eq!(herd.herd[COMET_INDEX].points(), 0);
        assert_eq!(herd.herd[DANCER_INDEX].points(), 1);

        assert_eq!(herd.race_winning_by_points(139), 139);
        assert_eq!(herd.herd[COMET_INDEX].points(), 0);
        assert_eq!(herd.herd[DANCER_INDEX].points(), 139);

        assert_eq!(herd.race_winning_by_points(140), 139);
        assert_eq!(herd.herd[COMET_INDEX].points(), 1);
        assert_eq!(herd.herd[DANCER_INDEX].points(), 139);

        assert_eq!(herd.race_winning_by_points(1000), 689);
        assert_eq!(herd.herd[COMET_INDEX].points(), 312);
        assert_eq!(herd.herd[DANCER_INDEX].points(), 689);
    }
}
