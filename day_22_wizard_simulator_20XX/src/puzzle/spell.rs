use crate::puzzle::{boss::Boss, wizard::Wizard};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellType {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spell {
    spell_type: SpellType,
    mana_cost: usize,
    effect_rounds: usize,
}

impl Spell {
    pub fn new(spell_type: SpellType) -> Self {
        match spell_type {
            SpellType::MagicMissile => Self {
                spell_type,
                mana_cost: 53,
                effect_rounds: 0,
            },
            SpellType::Drain => Self {
                spell_type,
                mana_cost: 73,
                effect_rounds: 0,
            },
            SpellType::Shield => Self {
                spell_type,
                mana_cost: 113,
                effect_rounds: 6,
            },
            SpellType::Poison => Self {
                spell_type,
                mana_cost: 173,
                effect_rounds: 6,
            },
            SpellType::Recharge => Self {
                spell_type,
                mana_cost: 229,
                effect_rounds: 5,
            },
        }
    }

    pub fn spell_type(&self) -> &SpellType {
        &self.spell_type
    }

    pub fn cast(&mut self, wizard: &mut Wizard, boss: &mut Boss) {
        // Every spell cast drains mana
        wizard.drain_mana(self.mana_cost);

        // Apply immediate action
        match self.spell_type {
            SpellType::MagicMissile => {
                boss.take_damage(4);
            }
            SpellType::Drain => {
                boss.take_damage(2);
                wizard.heal(2);
            }
            SpellType::Shield => {}
            SpellType::Poison => {}
            SpellType::Recharge => {}
        }
    }

    pub fn apply_effect(&mut self, wizard: &mut Wizard, boss: &mut Boss) {
        // Check for correctness
        if self.effect_rounds == 0 {
            panic!("Trying to apply effect for expired spell [{self:?}]");
        }

        // Decrease effect count
        self.effect_rounds -= 1;

        // Apply effect
        match self.spell_type {
            SpellType::MagicMissile => {}
            SpellType::Drain => {}
            SpellType::Shield => {
                wizard.receive_armor(7);
            }
            SpellType::Poison => {
                boss.take_damage(3);
            }
            SpellType::Recharge => {
                wizard.receive_mana(101);
            }
        }
    }

    pub fn is_expired(&self) -> bool {
        self.effect_rounds == 0
    }

    pub fn will_be_expired(&self) -> bool {
        self.effect_rounds == 1
    }
}
