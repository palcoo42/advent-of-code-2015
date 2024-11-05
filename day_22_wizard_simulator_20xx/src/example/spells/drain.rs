use crate::example::{boss::Boss, wizard::Wizard};

use super::{spell::Spell, spell_type::SpellType};

const MANA_COST: u32 = 73;
const INSTANT_DAMAGE: u32 = 2;
const INSTANT_HEAL: u32 = 2;

#[derive(Debug, Default, Clone)]
pub struct Drain {}

impl Drain {
    pub fn new() -> Self {
        Self {}
    }
}

impl Spell for Drain {
    fn get_spell_type(&self) -> SpellType {
        SpellType::Drain
    }

    fn get_mana_cost(&self) -> u32 {
        MANA_COST
    }

    fn clone_box(&self) -> Box<dyn Spell> {
        Box::new(self.clone())
    }

    fn apply_instant_effect(&mut self, wizard: &mut Wizard, boss: &mut Boss) {
        wizard.decrease_mana(MANA_COST);
        wizard.increase_hit_points(INSTANT_HEAL);
        boss.decrease_hit_points(INSTANT_DAMAGE);
    }
}
