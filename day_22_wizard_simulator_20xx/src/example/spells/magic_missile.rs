use crate::example::{boss::Boss, wizard::Wizard};

use super::{spell::Spell, spell_type::SpellType};

const MANA_COST: u32 = 53;
const INSTANT_DAMAGE: u32 = 4;

#[derive(Debug, Default, Clone)]
pub struct MagicMissile {}

impl MagicMissile {
    pub fn new() -> Self {
        Self {}
    }
}

impl Spell for MagicMissile {
    fn get_spell_type(&self) -> SpellType {
        SpellType::MagicMissile
    }

    fn get_mana_cost(&self) -> u32 {
        MANA_COST
    }

    fn clone_box(&self) -> Box<dyn Spell> {
        Box::new(self.clone())
    }

    fn apply_instant_effect(&mut self, wizard: &mut Wizard, boss: &mut Boss) {
        wizard.decrease_mana(MANA_COST);
        boss.decrease_hit_points(INSTANT_DAMAGE);
    }
}
