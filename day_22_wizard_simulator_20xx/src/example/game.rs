use super::{
    active_spells::ActiveSpells,
    boss::Boss,
    difficulty::Difficulty,
    spells::{spell::Spell, spell_type::SpellType},
    winner::Winner,
    wizard::Wizard,
};

#[derive(Debug, Clone)]
pub struct Game {
    wizard: Wizard,
    boss: Boss,
    active_spells: ActiveSpells,
    history: Vec<SpellType>,
    difficulty: Difficulty,
}

impl Game {
    pub fn new(wizard: Wizard, boss: Boss, difficulty: Difficulty) -> Self {
        Self {
            wizard,
            boss,
            active_spells: ActiveSpells::new(),
            history: Vec::new(),
            difficulty,
        }
    }

    // Play next round of the game with provided spell
    pub fn next_round(&mut self, spell: Box<dyn Spell>) -> Option<Winner> {
        // Player starts the first
        if let Some(winner) = self.player_turn(spell) {
            return Some(winner);
        }

        // Boss is the next
        if let Some(winner) = self.boss_turn() {
            return Some(winner);
        }

        // No winner yet
        None
    }

    fn player_turn(&mut self, mut spell: Box<dyn Spell>) -> Option<Winner> {
        // Part 2: Hard difficulty
        if self.difficulty == Difficulty::Hard {
            // Decrease player's hit point by 1
            self.wizard.decrease_hit_points(1);

            // Check end of the game
            if let Some(winner) = self.check_winner() {
                return Some(winner);
            }
        }

        // Update history
        self.history.push(spell.get_spell_type());

        // As a first step apply active spells
        self.active_spells
            .apply_effects(&mut self.wizard, &mut self.boss);

        if let Some(winner) = self.check_winner() {
            return Some(winner);
        }

        // If player cannot cast the spell it is considered as a failure
        if self.wizard.get_mana() < spell.get_mana_cost() {
            return Some(Winner::Boss);
        }

        spell.apply_instant_effect(&mut self.wizard, &mut self.boss);

        // Add spell to the active spells if applicable
        if spell.get_remaining_turns() > 0 {
            self.active_spells.add_spell(spell);
        }

        self.check_winner()
    }

    fn boss_turn(&mut self) -> Option<Winner> {
        // As a first step apply active spells
        self.active_spells
            .apply_effects(&mut self.wizard, &mut self.boss);

        if let Some(winner) = self.check_winner() {
            return Some(winner);
        }

        // Apply damage to the wizard, damage is at least 1
        let damage = std::cmp::max(
            1,
            self.boss.get_damage() as i32 - self.active_spells.get_spells_armor() as i32,
        ) as u32;

        self.wizard.decrease_hit_points(damage);

        self.check_winner()
    }

    fn check_winner(&self) -> Option<Winner> {
        if self.wizard.get_hit_points() == 0 {
            return Some(Winner::Boss);
        }

        if self.boss.get_hit_points() == 0 {
            return Some(Winner::Wizard);
        }

        None
    }

    // Provide list of allowed spells to the simulation
    pub fn get_allowed_spells(&self, max_mana: u32) -> Vec<Box<dyn Spell>> {
        self.active_spells
            .get_allowed_spells()
            .into_iter()
            .filter(|s| s.get_mana_cost() <= max_mana)
            .collect::<Vec<_>>()
    }

    pub fn get_spent_mana(&self) -> u32 {
        self.wizard.get_spent_mana()
    }

    pub fn get_history(&self) -> &Vec<SpellType> {
        &self.history
    }
}

#[cfg(test)]
mod tests {
    use crate::example::spells::{
        drain::Drain, magic_missile::MagicMissile, poison::Poison, recharge::Recharge,
        shield::Shield, spell_type::SpellType,
    };

    use super::*;

    fn create_game_1() -> Game {
        Game::new(Wizard::new(10, 250), Boss::new(13, 8), Difficulty::Normal)
    }

    fn create_game_2() -> Game {
        Game::new(Wizard::new(10, 250), Boss::new(14, 8), Difficulty::Normal)
    }

    #[test]
    fn test_check_winner_wizard() {
        let mut game = create_game_1();

        assert_eq!(game.check_winner(), None);

        game.boss.decrease_hit_points(game.boss.get_hit_points());
        assert_eq!(game.check_winner(), Some(Winner::Wizard));
    }

    #[test]
    fn test_check_winner_boss() {
        let mut game = create_game_1();

        assert_eq!(game.check_winner(), None);

        game.wizard
            .decrease_hit_points(game.wizard.get_hit_points());
        assert_eq!(game.check_winner(), Some(Winner::Boss));
    }

    #[test]
    fn test_game_1() {
        let mut game = create_game_1();

        assert_eq!(game.wizard.get_hit_points(), 10);
        assert_eq!(game.active_spells.get_spells_armor(), 0);
        assert_eq!(game.wizard.get_mana(), 250);
        assert_eq!(game.active_spells.len(), 0);
        assert_eq!(game.boss.get_hit_points(), 13);

        // Round #1: Player casts Poison
        assert_eq!(game.next_round(Box::new(Poison::new())), None);

        assert_eq!(game.wizard.get_hit_points(), 2);
        assert_eq!(game.active_spells.get_spells_armor(), 0);
        assert_eq!(game.wizard.get_mana(), 77);
        assert_eq!(game.active_spells.len(), 1);
        assert_eq!(
            game.active_spells
                .get_spell(0)
                .unwrap()
                .get_remaining_turns(),
            5
        );
        assert_eq!(game.boss.get_hit_points(), 10);

        // Round #2: Player casts MagicMissile
        assert_eq!(
            game.next_round(Box::new(MagicMissile::new())),
            Some(Winner::Wizard)
        );

        assert_eq!(game.wizard.get_hit_points(), 2);
        assert_eq!(game.active_spells.get_spells_armor(), 0);
        assert_eq!(game.wizard.get_mana(), 24);
        assert_eq!(game.active_spells.len(), 1);
        assert_eq!(
            game.active_spells.get_spell(0).unwrap().get_spell_type(),
            SpellType::Poison
        );
        assert_eq!(
            game.active_spells
                .get_spell(0)
                .unwrap()
                .get_remaining_turns(),
            3
        );
        assert_eq!(game.boss.get_hit_points(), 0);
    }

    #[test]
    fn test_game_2() {
        let mut game = create_game_2();

        assert_eq!(game.wizard.get_hit_points(), 10);
        assert_eq!(game.active_spells.get_spells_armor(), 0);
        assert_eq!(game.wizard.get_mana(), 250);
        assert_eq!(game.active_spells.len(), 0);
        assert_eq!(game.boss.get_hit_points(), 14);

        // Round #1: Player casts Recharge
        assert_eq!(game.next_round(Box::new(Recharge::new())), None);

        assert_eq!(game.wizard.get_hit_points(), 2);
        assert_eq!(game.active_spells.get_spells_armor(), 0);
        assert_eq!(game.wizard.get_mana(), 122);
        assert_eq!(game.active_spells.len(), 1);
        assert_eq!(
            game.active_spells.get_spell(0).unwrap().get_spell_type(),
            SpellType::Recharge
        );
        assert_eq!(
            game.active_spells
                .get_spell(0)
                .unwrap()
                .get_remaining_turns(),
            4
        );
        assert_eq!(game.boss.get_hit_points(), 14);

        // Round #2: Player casts Shield
        assert_eq!(game.next_round(Box::new(Shield::new())), None);

        assert_eq!(game.wizard.get_hit_points(), 1);
        assert_eq!(game.active_spells.get_spells_armor(), 7);
        assert_eq!(game.wizard.get_mana(), 211);
        assert_eq!(game.active_spells.len(), 2);
        assert_eq!(
            game.active_spells.get_spell(0).unwrap().get_spell_type(),
            SpellType::Recharge
        );
        assert_eq!(
            game.active_spells
                .get_spell(0)
                .unwrap()
                .get_remaining_turns(),
            2
        );
        assert_eq!(
            game.active_spells.get_spell(1).unwrap().get_spell_type(),
            SpellType::Shield
        );
        assert_eq!(
            game.active_spells
                .get_spell(1)
                .unwrap()
                .get_remaining_turns(),
            5
        );
        assert_eq!(game.boss.get_hit_points(), 14);

        // Round #3: Player casts Drain
        assert_eq!(game.next_round(Box::new(Drain::new())), None);

        assert_eq!(game.wizard.get_hit_points(), 2);
        assert_eq!(game.active_spells.get_spells_armor(), 7);
        assert_eq!(game.wizard.get_mana(), 340);
        assert_eq!(game.active_spells.len(), 1);
        assert_eq!(
            game.active_spells.get_spell(0).unwrap().get_spell_type(),
            SpellType::Shield
        );
        assert_eq!(
            game.active_spells
                .get_spell(0)
                .unwrap()
                .get_remaining_turns(),
            3
        );
        assert_eq!(game.boss.get_hit_points(), 12);

        // Round #4: Player casts Poison
        assert_eq!(game.next_round(Box::new(Poison::new())), None);

        assert_eq!(game.wizard.get_hit_points(), 1);
        assert_eq!(game.active_spells.get_spells_armor(), 7);
        assert_eq!(game.wizard.get_mana(), 167);
        assert_eq!(game.active_spells.len(), 2);
        assert_eq!(
            game.active_spells.get_spell(0).unwrap().get_spell_type(),
            SpellType::Shield
        );
        assert_eq!(
            game.active_spells
                .get_spell(0)
                .unwrap()
                .get_remaining_turns(),
            1
        );
        assert_eq!(
            game.active_spells.get_spell(1).unwrap().get_spell_type(),
            SpellType::Poison
        );
        assert_eq!(
            game.active_spells
                .get_spell(1)
                .unwrap()
                .get_remaining_turns(),
            5
        );
        assert_eq!(game.boss.get_hit_points(), 9);

        // Round #5: Player casts MagicMissile
        assert_eq!(
            game.next_round(Box::new(MagicMissile::new())),
            Some(Winner::Wizard)
        );

        assert_eq!(game.wizard.get_hit_points(), 1);
        assert_eq!(game.active_spells.get_spells_armor(), 0);
        assert_eq!(game.wizard.get_mana(), 114);
        assert_eq!(game.active_spells.len(), 1);
        assert_eq!(
            game.active_spells.get_spell(0).unwrap().get_spell_type(),
            SpellType::Poison
        );
        assert_eq!(
            game.active_spells
                .get_spell(0)
                .unwrap()
                .get_remaining_turns(),
            3
        );
        assert_eq!(game.boss.get_hit_points(), 0);
    }
}
