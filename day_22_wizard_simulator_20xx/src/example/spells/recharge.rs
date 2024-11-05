use crate::example::{boss::Boss, wizard::Wizard};

use super::{spell::Spell, spell_type::SpellType};

const MANA_COST: u32 = 229;
const ROUND_MANA_RECHARGE: u32 = 101;
const EFFECT_TURNS: u32 = 5;

#[derive(Debug, Default, Clone)]
pub struct Recharge {
    remaining_turns: u32,
}

impl Recharge {
    pub fn new() -> Self {
        Self {
            remaining_turns: EFFECT_TURNS,
        }
    }
}

impl Spell for Recharge {
    fn get_spell_type(&self) -> SpellType {
        SpellType::Recharge
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

    fn apply_round_effect(&mut self, wizard: &mut Wizard, _boss: &mut Boss) {
        if self.remaining_turns == 0 {
            panic!("Remaining turns are already at zero");
        }

        self.remaining_turns -= 1;
        wizard.increase_mana(ROUND_MANA_RECHARGE);
    }
}
