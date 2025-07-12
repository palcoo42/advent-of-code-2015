use std::error::Error;

use puzzler::env::project;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::aunt::Aunt;

pub struct Solution {
    aunts: Vec<Aunt>,
    gifts: Vec<(String, usize)>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 16: Aunt Sue ---"
    }

    fn get_input_file_path(&self) -> Option<std::path::PathBuf> {
        Some(
            project::get_project_file("../input/day_16.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_16.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.aunts = Parser::parse_lines_with_regex(
            lines,
            r"^Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)",
            |params| {
                if params.len() != 7 {
                    return Err(format!("Expected exactly 7 parameters '{params:?}'").into());
                }

                let mut params_iter = params.into_iter();

                let id = params_iter
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .map_err(|err| format!("Failed to convert 'id' to usize [{err:?}]"))?;

                let mut compounds = Vec::new();

                for _ in 0..3 {
                    let comp = params_iter.next().unwrap();

                    let count = params_iter
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .map_err(|err| format!("Failed to convert 'count' to usize [{err:?}]"))?;

                    compounds.push((comp, count));
                }

                Ok(Aunt::new(id, compounds))
            },
        )?;

        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let aunt_nr = self.find_aunt_with_gift(&self.gifts).unwrap();
        Ok(aunt_nr.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let aunt_nr = self.find_aunt_with_gift_real(&self.gifts).unwrap();
        Ok(aunt_nr.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            aunts: Vec::new(),
            gifts: vec![
                (String::from("children"), 3),
                (String::from("cats"), 7),
                (String::from("samoyeds"), 2),
                (String::from("pomeranians"), 3),
                (String::from("akitas"), 0),
                (String::from("vizslas"), 0),
                (String::from("goldfish"), 5),
                (String::from("trees"), 3),
                (String::from("cars"), 2),
                (String::from("perfumes"), 1),
            ],
        }
    }

    fn find_aunt_with_gift(&self, gifts: &[(String, usize)]) -> Option<usize> {
        let aunts = self
            .aunts
            .iter()
            .filter(|aunt| {
                gifts
                    .iter()
                    .all(|(name, count)| match aunt.get_compound(name) {
                        Some(number) => number == count,
                        None => true,
                    })
            })
            .collect::<Vec<_>>();

        if aunts.len() != 1 {
            panic!("More than one aunt found '{aunts:?}'");
        }

        Some(aunts[0].get_id())
    }

    fn find_aunt_with_gift_real(&self, gifts: &[(String, usize)]) -> Option<usize> {
        let aunts = self
            .aunts
            .iter()
            .filter(|aunt| {
                gifts
                    .iter()
                    .all(|(name, count)| match aunt.get_compound(name) {
                        Some(number) => match name.as_str() {
                            "cats" | "trees" => number > count,
                            "pomemarian" | "goldfish" => number < count,
                            _ => number == count,
                        },
                        None => true,
                    })
            })
            .collect::<Vec<_>>();

        if aunts.len() != 1 {
            panic!("More than one aunt found '{aunts:?}'");
        }

        Some(aunts[0].get_id())
    }
}

#[cfg(test)]
mod tests {
    use puzzler::puzzler::puzzle::Puzzle;

    use crate::puzzle::solution::Solution;

    fn get_puzzle() -> Solution {
        let mut solution = Solution::new();

        solution
            .parse_input_file()
            .unwrap_or_else(|err| panic!("Failed to parse input file [{err}]"));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "213");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "323");
    }
}
