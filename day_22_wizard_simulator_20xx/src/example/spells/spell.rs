use std::fmt::Debug;

use crate::example::{boss::Boss, wizard::Wizard};

use super::spell_type::SpellType;

pub trait Spell: Debug {
    fn get_spell_type(&self) -> SpellType;

    fn get_mana_cost(&self) -> u32;

    fn clone_box(&self) -> Box<dyn Spell>;

    fn get_armor(&self) -> u32 {
        0
    }

    fn get_remaining_turns(&self) -> u32 {
        0
    }

    fn apply_instant_effect(&mut self, wizard: &mut Wizard, boss: &mut Boss);

    fn apply_round_effect(&mut self, _wizard: &mut Wizard, _boss: &mut Boss) {}
}

// Implement clone for Box<>
impl Clone for Box<dyn Spell> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
