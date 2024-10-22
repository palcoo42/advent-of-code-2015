use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use super::city::City;

type Distance = u32;
type Destination = HashMap<City, Distance>;

#[derive(Default)]
pub struct Locations {
    cities: HashMap<City, Destination>,
}

impl Locations {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, src: &str, dst: &str, distance: u32) {
        // Insert first pair
        let destinations = self.cities.entry(City::new(src)).or_default();
        destinations.insert(City::new(dst), distance);

        // Insert second pair
        let destinations = self.cities.entry(City::new(dst)).or_default();
        destinations.insert(City::new(src), distance);
    }

    pub fn find_shortest_path(&self) -> u32 {
        // Find all permutations and count the distances
        let cities = self.cities.keys().collect::<HashSet<_>>();
        let permutations = cities.iter().permutations(self.cities.len());

        permutations
            .map(|p| {
                let cities = p.iter().map(|city| city.name.as_str()).collect::<Vec<_>>();
                self.find_distance(&cities)
            })
            .min()
            .unwrap_or_else(|| panic!("Failed to find shortest distance"))
    }

    pub fn find_longest_path(&self) -> u32 {
        // Find all permutations and count the distances
        let cities = self.cities.keys().collect::<HashSet<_>>();
        let permutations = cities.iter().permutations(self.cities.len());

        permutations
            .map(|p| {
                let cities = p.iter().map(|city| city.name.as_str()).collect::<Vec<_>>();
                self.find_distance(&cities)
            })
            .max()
            .unwrap_or_else(|| panic!("Failed to find longest distance"))
    }

    fn find_distance(&self, cities: &[&str]) -> u32 {
        // Create a pairs of cities to form the road
        let pairs = (0..cities.len() - 1)
            .map(|idx| (cities[idx], cities[idx + 1]))
            .collect::<Vec<_>>();

        let mut distance = 0;

        for (src, dst) in pairs {
            // Find all destinations associated with 'src'
            let destinations = self
                .cities
                .get(&City::new(src))
                .unwrap_or_else(|| panic!("City '{}' not found within cities", src));

            // Find concrete destination associated with 'src' and 'dst'
            let destination = destinations.get(&City::new(dst)).unwrap_or_else(|| {
                panic!("City '{}' not found within destinations of '{}'", dst, src)
            });

            distance += destination;
        }

        distance
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn create_locations() -> Locations {
        let mut locations = Locations::new();
        locations.insert("London", "Dublin", 464);
        locations.insert("London", "Belfast", 518);
        locations.insert("Dublin", "Belfast", 141);
        locations
    }

    #[test]
    fn test_find_shortest_path() {
        let locations = create_locations();
        assert_eq!(locations.find_shortest_path(), 605);
    }

    #[test]
    fn test_find_longest_path() {
        let locations = create_locations();
        assert_eq!(locations.find_longest_path(), 982);
    }

    #[test]
    fn test_find_distance() {
        let locations = create_locations();

        assert_eq!(
            locations.find_distance(&["Dublin", "London", "Belfast"]),
            982
        );

        assert_eq!(
            locations.find_distance(&["London", "Dublin", "Belfast"]),
            605
        );

        assert_eq!(
            locations.find_distance(&["London", "Belfast", "Dublin"]),
            659
        );

        assert_eq!(
            locations.find_distance(&["Dublin", "Belfast", "London"]),
            659
        );

        assert_eq!(
            locations.find_distance(&["Belfast", "Dublin", "London"]),
            605
        );

        assert_eq!(
            locations.find_distance(&["Belfast", "London", "Dublin"]),
            982
        );
    }
}
