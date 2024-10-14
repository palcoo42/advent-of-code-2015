#[derive(Debug, PartialEq)]
pub struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    // Extract dimensions from string LxWxH (Length - Width - Height)
    pub fn new(dimensions: &str) -> Option<Self> {
        let parts = dimensions.split('x').collect::<Vec<_>>();

        if parts.len() != 3 {
            eprintln!("Dimensions should contain exactly 3 parts, {}", dimensions);
            return None;
        }

        let length = parts[0].parse::<u32>().ok()?;
        let width = parts[1].parse::<u32>().ok()?;
        let height = parts[2].parse::<u32>().ok()?;

        Some(Self {
            length,
            width,
            height,
        })
    }

    pub fn wrapping_paper(&self) -> u32 {
        // Find out two smallest numbers
        let mut numbers = [self.length, self.width, self.height];
        numbers.sort();

        // Calculate square feet + area of the smallest side
        2 * (self.length * self.width + self.length * self.height + self.width * self.height)
            + numbers[0] * numbers[1]
    }

    pub fn ribbon(&self) -> u32 {
        // Find out two smallest numbers
        let mut numbers = [self.length, self.width, self.height];
        numbers.sort();

        // Calculate ribbon size
        2 * (numbers[0] + numbers[1]) + self.length * self.width * self.height
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Some(Present {
                length: 1,
                width: 2,
                height: 3
            }),
            Present::new("1x2x3")
        );
    }

    #[test]
    fn test_new_non_numbers() {
        assert_eq!(None, Present::new("ax2x3"));
        assert_eq!(None, Present::new("1xbx3"));
        assert_eq!(None, Present::new("1x2xc"));
        assert_eq!(None, Present::new("axbbxcccc"));
    }

    #[test]
    fn test_new_too_few_fields() {
        assert_eq!(None, Present::new("1"));
        assert_eq!(None, Present::new("1x5"));
    }

    #[test]
    fn test_new_too_many_fields() {
        assert_eq!(None, Present::new("1x2x3x4"));
    }

    #[test]
    fn test_wrapping_paper() {
        assert_eq!(
            Present {
                length: 2,
                width: 3,
                height: 4
            }
            .wrapping_paper(),
            58
        );

        assert_eq!(
            Present {
                length: 1,
                width: 1,
                height: 10
            }
            .wrapping_paper(),
            43
        );
    }

    #[test]
    fn test_ribbon() {
        assert_eq!(
            Present {
                length: 2,
                width: 3,
                height: 4
            }
            .ribbon(),
            34
        );

        assert_eq!(
            Present {
                length: 1,
                width: 1,
                height: 10
            }
            .ribbon(),
            14
        );
    }
}
