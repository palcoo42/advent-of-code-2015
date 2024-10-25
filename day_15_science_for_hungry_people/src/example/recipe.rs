use iter_tools::Itertools;

use super::ingredient::Ingredient;

pub struct Recipe {
    ingredients: Vec<Ingredient>,
}

impl Recipe {
    pub fn new(ingredients: Vec<Ingredient>) -> Self {
        Self { ingredients }
    }

    pub fn highest_score(&self) -> u32 {
        const TEA_SPOONS: i32 = 100;

        // Use only permutation where sum of all spoons is equal to TEA_SPOONS
        let permutations = (0..=TEA_SPOONS)
            .permutations(self.ingredients.len())
            .filter(|values| values.iter().sum::<i32>() == TEA_SPOONS);

        permutations
            .into_iter()
            .map(|spoons| self.calculate_score(&spoons))
            .max()
            .expect("Failed to calculate maximum")
    }

    pub fn highest_score_with_calories(&self, calories: u32) -> u32 {
        const TEA_SPOONS: i32 = 100;

        // Use only permutation where sum of all spoons is equal to TEA_SPOONS
        let permutations = (0..=TEA_SPOONS)
            .permutations(self.ingredients.len())
            .filter(|values| values.iter().sum::<i32>() == TEA_SPOONS);

        permutations
            .into_iter()
            .filter_map(|spoons| {
                if self.calculate_calories(&spoons) == calories {
                    Some(self.calculate_score(&spoons))
                } else {
                    None
                }
            })
            .max()
            .expect("Failed to calculate maximum")
    }

    fn calculate_score(&self, spoons: &[i32]) -> u32 {
        if self.ingredients.len() != spoons.len() {
            panic!("Length of ingredients != spoons");
        }

        let capacity = self
            .ingredients
            .iter()
            .zip(spoons.iter())
            .map(|(ingredient, spoons)| ingredient.capacity * spoons)
            .sum::<i32>();

        let durability = self
            .ingredients
            .iter()
            .zip(spoons.iter())
            .map(|(ingredient, spoons)| ingredient.durability * spoons)
            .sum::<i32>();

        let flavor = self
            .ingredients
            .iter()
            .zip(spoons.iter())
            .map(|(ingredient, spoons)| ingredient.flavor * spoons)
            .sum::<i32>();

        let texture = self
            .ingredients
            .iter()
            .zip(spoons.iter())
            .map(|(ingredient, spoons)| ingredient.texture * spoons)
            .sum::<i32>();

        // Negative value translates to 0. As a result multiple will be also 0.
        if capacity.is_negative()
            || durability.is_negative()
            || flavor.is_negative()
            || texture.is_negative()
        {
            0
        } else {
            (capacity * durability * flavor * texture) as u32
        }
    }

    fn calculate_calories(&self, spoons: &[i32]) -> u32 {
        if self.ingredients.len() != spoons.len() {
            panic!("Length of ingredients != spoons");
        }

        self.ingredients
            .iter()
            .zip(spoons.iter())
            .map(|(ingredient, spoons)| ingredient.calories * (*spoons as u32))
            .sum::<u32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_recipe() -> Recipe {
        Recipe::new(vec![
            Ingredient {
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8,
            },
            Ingredient {
                capacity: 2,
                durability: 3,
                flavor: -2,
                texture: -1,
                calories: 3,
            },
        ])
    }

    #[test]
    fn test_highest_score() {
        let recipe = create_recipe();

        assert_eq!(recipe.highest_score(), 62842880);
    }

    #[test]
    fn test_calculate_score() {
        let recipe = create_recipe();
        let spoons = [44, 56];

        assert_eq!(recipe.calculate_score(&spoons), 62842880);
    }

    #[test]
    fn test_highest_score_with_calories() {
        let recipe = create_recipe();

        assert_eq!(recipe.highest_score_with_calories(500), 57600000);
    }

    #[test]
    fn test_calculate_calories() {
        let recipe = create_recipe();
        let spoons = [40, 60];

        assert_eq!(recipe.calculate_calories(&spoons), 500);
    }
}
