// Part 1
const PRESENTS_PER_ELF: usize = 10;

// Part 2
const PRESENTS_PER_ELF_FINITE: usize = 11;
const HOUSES_FINITE: usize = 50;

pub struct Street {
    street: Vec<usize>,
}

impl Street {
    pub fn new(count: usize) -> Self {
        Self {
            // To ease handling do not use index 0 (house 0)
            street: vec![0; count + 1],
        }
    }

    pub fn deliver_presents(&mut self, presents: usize) -> Option<usize> {
        // Reset all counts to zero
        self.street.iter_mut().for_each(|count| *count = 0);

        // Go though every elf (up to number of street and deliver presents)
        for elf in 1..self.street.len() {
            for house_id in (elf..self.street.len()).step_by(elf) {
                self.street[house_id] += elf * PRESENTS_PER_ELF;
            }
        }

        // Find first house which has at least 'presents' delivered
        self.street
            .iter()
            .enumerate()
            .find(|(_idx, &count)| count >= presents)
            .map(|(idx, _count)| idx)
    }

    pub fn deliver_presents_finite(&mut self, presents: usize) -> Option<usize> {
        // Reset all counts to zero
        self.street.iter_mut().for_each(|count| *count = 0);

        // Go though every elf (up to number of street and deliver presents)
        for elf in 1..self.street.len() {
            for house_id in (elf..self.street.len()).step_by(elf).take(HOUSES_FINITE) {
                self.street[house_id] += elf * PRESENTS_PER_ELF_FINITE;
            }
        }

        // Find first house which has at least 'presents' delivered
        self.street
            .iter()
            .enumerate()
            .find(|(_idx, &count)| count >= presents)
            .map(|(idx, _count)| idx)
    }
}

#[cfg(test)]
mod tests {
    use super::Street;

    #[test]
    fn test_deliver_presents() {
        let mut street = Street::new(10);

        assert_eq!(street.deliver_presents(10), Some(1));
        assert_eq!(street.deliver_presents(30), Some(2));
        assert_eq!(street.deliver_presents(40), Some(3));
        assert_eq!(street.deliver_presents(70), Some(4));
        assert_eq!(street.deliver_presents(60), Some(4));
        assert_eq!(street.deliver_presents(120), Some(6));
        assert_eq!(street.deliver_presents(80), Some(6));
        assert_eq!(street.deliver_presents(121), Some(8));
        assert_eq!(street.deliver_presents(150), Some(8));
        assert_eq!(street.deliver_presents(130), Some(8));

        assert_eq!(street.street[0], 0);
        assert_eq!(street.street[1], 10);
        assert_eq!(street.street[2], 30);
        assert_eq!(street.street[3], 40);
        assert_eq!(street.street[4], 70);
        assert_eq!(street.street[5], 60);
        assert_eq!(street.street[6], 120);
        assert_eq!(street.street[7], 80);
        assert_eq!(street.street[8], 150);
        assert_eq!(street.street[9], 130);
        assert_eq!(street.street[10], 180);
    }
}
