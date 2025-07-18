use crate::puzzle::{
    boss::Boss,
    spell::{Spell, SpellType},
    winner::Winner,
    wizard::Wizard,
};

#[derive(Debug, Clone)]
pub struct Game {
    wizard: Wizard,
    boss: Boss,
    spells: Vec<Spell>,
    history: Vec<SpellType>,
    winner: Option<Winner>,
    hard_mode: bool,
}

impl Game {
    pub fn new(wizard: Wizard, boss: Boss, hard_mode: bool) -> Self {
        Self {
            wizard,
            boss,
            spells: Vec::new(),
            history: Vec::new(),
            winner: None,
            hard_mode,
        }
    }

    pub fn one_round(&mut self, mut spell: Spell) {
        // Player's round

        // Hardcore mode
        if self.hard_mode {
            self.wizard.take_damage_no_armor(1);

            if self.is_game_over() {
                return;
            }
        }

        self.apply_spells();

        if self.is_game_over() {
            return;
        }

        // Cast new spell
        self.history.push(spell.spell_type().clone());
        spell.cast(&mut self.wizard, &mut self.boss);
        if !spell.is_expired() {
            self.spells.push(spell);
        }

        if self.is_game_over() {
            return;
        }

        // Boss's round
        self.apply_spells();

        if self.is_game_over() {
            return;
        }

        self.wizard.take_damage(self.boss.damage);

        self.is_game_over();
    }

    fn apply_spells(&mut self) {
        // Reset old effects
        self.wizard.reset_spell_effects();

        // Apply spells
        self.spells
            .iter_mut()
            .for_each(|spell| spell.apply_effect(&mut self.wizard, &mut self.boss));

        // Remove expired spells
        self.spells.retain(|s| !s.is_expired())
    }

    fn is_game_over(&mut self) -> bool {
        // Check boss loss
        if self.boss.hit_points == 0 {
            self.winner = Some(Winner::Wizard);
            return true;
        }

        // Check wizard loss
        if self.wizard.hit_points == 0 || self.wizard.mana == 0 {
            self.winner = Some(Winner::Boss);
            return true;
        }

        false
    }

    pub fn get_castable_spells(&self) -> Vec<SpellType> {
        let spells = vec![
            SpellType::MagicMissile,
            SpellType::Drain,
            SpellType::Shield,
            SpellType::Poison,
            SpellType::Recharge,
        ];

        // TRICKY: This method is called before round_one is called. Therefore it is allowed
        // to include spells which have remaining duration 1 because these will be consumed.
        spells
            .into_iter()
            .filter(|spell| {
                let active = self.spells.iter().find(|s| spell == s.spell_type());

                match active {
                    Some(active) => active.will_be_expired(),
                    None => true,
                }
            })
            .collect()
    }

    pub fn get_wizard(&self) -> &Wizard {
        &self.wizard
    }

    pub fn get_winner(&self) -> &Option<Winner> {
        &self.winner
    }

    #[allow(unused)]
    pub fn get_history(&self) -> &Vec<SpellType> {
        &self.history
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn construct_game(hard_mode: bool) -> Game {
        let wizard = Wizard::new(10, 250);
        let boss = Boss::new(13, 8);
        Game::new(wizard, boss, hard_mode)
    }

    #[test]
    fn test_one_round_example_1() {
        let mut game = construct_game(false);

        game.one_round(Spell::new(SpellType::Poison));
        assert_eq!(game.get_winner(), &None);
        assert_eq!(game.wizard.hit_points, 2);
        assert_eq!(game.wizard.mana, 77);
        assert_eq!(game.boss.hit_points, 10);

        game.one_round(Spell::new(SpellType::MagicMissile));
        assert_eq!(game.get_winner(), &Some(Winner::Wizard));
        assert_eq!(game.wizard.hit_points, 2);
        assert_eq!(game.wizard.mana, 24);
        assert_eq!(game.boss.hit_points, 0);
    }

    #[test]
    fn test_one_round_example_2() {
        let mut game = construct_game(false);
        game.boss.hit_points = 14;

        game.one_round(Spell::new(SpellType::Recharge));
        assert_eq!(game.get_winner(), &None);
        assert_eq!(game.wizard.hit_points, 2);
        assert_eq!(game.wizard.mana, 122);
        assert_eq!(game.boss.hit_points, 14);

        game.one_round(Spell::new(SpellType::Shield));
        assert_eq!(game.get_winner(), &None);
        assert_eq!(game.wizard.hit_points, 1);
        assert_eq!(game.wizard.mana, 211);
        assert_eq!(game.boss.hit_points, 14);

        game.one_round(Spell::new(SpellType::Drain));
        assert_eq!(game.get_winner(), &None);
        assert_eq!(game.wizard.hit_points, 2);
        assert_eq!(game.wizard.mana, 340);
        assert_eq!(game.boss.hit_points, 12);

        game.one_round(Spell::new(SpellType::Poison));
        assert_eq!(game.get_winner(), &None);
        assert_eq!(game.wizard.hit_points, 1);
        assert_eq!(game.wizard.mana, 167);
        assert_eq!(game.boss.hit_points, 9);

        game.one_round(Spell::new(SpellType::MagicMissile));
        assert_eq!(game.get_winner(), &Some(Winner::Wizard));
        assert_eq!(game.wizard.hit_points, 1);
        assert_eq!(game.wizard.mana, 114);
        assert_eq!(game.boss.hit_points, 0);
    }

    #[test]
    fn test_one_round_hard_mode() {
        let mut game = construct_game(true);
        game.wizard.hit_points = 50;
        game.wizard.mana = 500;
        game.boss.hit_points = 55;
        game.boss.damage = 8;

        game.one_round(Spell::new(SpellType::Poison));
        assert_eq!(game.get_winner(), &None);
        assert_eq!(game.wizard.hit_points, 41);
        assert_eq!(game.wizard.mana, 327);
        assert_eq!(game.boss.hit_points, 52);

        game.one_round(Spell::new(SpellType::Drain));
        assert_eq!(game.get_winner(), &None);
        assert_eq!(game.wizard.hit_points, 34);
        assert_eq!(game.wizard.mana, 254);
        assert_eq!(game.boss.hit_points, 44);

        game.one_round(Spell::new(SpellType::Recharge));
        assert_eq!(game.get_winner(), &None);
        assert_eq!(game.wizard.hit_points, 25);
        assert_eq!(game.wizard.mana, 126);
        assert_eq!(game.boss.hit_points, 38);

        game.one_round(Spell::new(SpellType::Poison));
        assert_eq!(game.get_winner(), &None);
        assert_eq!(game.wizard.hit_points, 16);
        assert_eq!(game.wizard.mana, 155);
        assert_eq!(game.boss.hit_points, 32);

        game.one_round(Spell::new(SpellType::Shield));
        assert_eq!(game.get_winner(), &None);
        assert_eq!(game.wizard.hit_points, 14);
        assert_eq!(game.wizard.mana, 244);
        assert_eq!(game.boss.hit_points, 26);

        game.one_round(Spell::new(SpellType::Recharge));
        assert_eq!(game.get_winner(), &None);
        assert_eq!(game.wizard.hit_points, 12);
        assert_eq!(game.wizard.mana, 116);
        assert_eq!(game.boss.hit_points, 20);

        game.one_round(Spell::new(SpellType::Poison));
        assert_eq!(game.get_winner(), &None);
        assert_eq!(game.wizard.hit_points, 10);
        assert_eq!(game.wizard.mana, 145);
        assert_eq!(game.boss.hit_points, 14);

        game.one_round(Spell::new(SpellType::Drain));
        assert_eq!(game.get_winner(), &None);
        assert_eq!(game.wizard.hit_points, 3);
        assert_eq!(game.wizard.mana, 274);
        assert_eq!(game.boss.hit_points, 6);

        game.one_round(Spell::new(SpellType::MagicMissile));
        assert_eq!(game.get_winner(), &Some(Winner::Wizard));
        assert_eq!(game.wizard.hit_points, 2);
        assert_eq!(game.wizard.mana, 221);
        assert_eq!(game.boss.hit_points, 0);

        assert_eq!(game.wizard.spent_mana, 1289);
    }
}
