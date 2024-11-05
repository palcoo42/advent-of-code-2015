use crate::example::{boss::Boss, wizard::Wizard};

use super::{spell::Spell, spell_type::SpellType};

const MANA_COST: u32 = 173;
const ROUND_DAMAGE: u32 = 3;
const EFFECT_TURNS: u32 = 6;

#[derive(Debug, Default, Clone)]
pub struct Poison {
    remaining_turns: u32,
}

impl Poison {
    pub fn new() -> Self {
        Self {
            remaining_turns: EFFECT_TURNS,
        }
    }
}

impl Spell for Poison {
    fn get_spell_type(&self) -> SpellType {
        SpellType::Poison
    }

    fn get_mana_cost(&self) -> u32 {
        MANA_COST
    }

    fn clone_box(&self) -> Box<dyn Spell> {
        Box::new(self.clone())
    }

    fn get_remaining_turns(&self) -> u32 {
        self.remaining_turns
    }

    fn apply_instant_effect(&mut self, wizard: &mut Wizard, _boss: &mut Boss) {
        wizard.decrease_mana(MANA_COST);
    }

    fn apply_round_effect(&mut self, _wizard: &mut Wizard, boss: &mut Boss) {
        if self.remaining_turns == 0 {
            panic!("Remaining turns are already at zero");
        }

        self.remaining_turns -= 1;
        boss.decrease_hit_points(ROUND_DAMAGE);
    }
}
