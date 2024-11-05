use super::{
    boss::Boss,
    spells::{
        drain::Drain, magic_missile::MagicMissile, poison::Poison, recharge::Recharge,
        shield::Shield, spell::Spell,
    },
    wizard::Wizard,
};

#[derive(Debug, Default, Clone)]
pub struct ActiveSpells {
    effects: Vec<Box<dyn Spell>>,
}

impl ActiveSpells {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.effects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.effects.len() == 0
    }

    pub fn get_allowed_spells(&self) -> Vec<Box<dyn Spell>> {
        let mut spells: Vec<Box<dyn Spell>> = vec![
            Box::new(MagicMissile::new()),
            Box::new(Drain::new()),
            Box::new(Shield::new()),
            Box::new(Poison::new()),
            Box::new(Recharge::new()),
        ];

        // Allow only spells which are not active or which ends this turn
        spells.retain(|spell| {
            let effect = self
                .effects
                .iter()
                .find(|effect| effect.get_spell_type() == spell.get_spell_type());

            match effect {
                Some(spell) => spell.get_remaining_turns() <= 1,
                None => true,
            }
        });

        spells
    }

    pub fn add_spell(&mut self, spell: Box<dyn Spell>) {
        // Invariant check
        if self
            .effects
            .iter()
            .any(|s| s.get_spell_type() == spell.get_spell_type())
        {
            panic!(
                "Active spells '{:?}' already contains spell '{:?}'",
                self.effects, spell
            );
        }

        self.effects.push(spell);
    }

    pub fn apply_effects(&mut self, wizard: &mut Wizard, boss: &mut Boss) {
        // Apply effects
        for effect in self.effects.iter_mut() {
            effect.apply_round_effect(wizard, boss);
        }

        // Remove effects which are expired
        self.effects.retain(|s| s.get_remaining_turns() > 0);
    }

    pub fn get_spells_armor(&self) -> u32 {
        self.effects.iter().map(|s| s.get_armor()).sum()
    }

    pub fn get_spell(&self, index: usize) -> Option<&Box<dyn Spell>> {
        self.effects.get(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::example::{
        boss::Boss,
        spells::{
            drain::Drain, magic_missile::MagicMissile, poison::Poison, spell_type::SpellType,
        },
        wizard::Wizard,
    };

    use super::*;

    fn create_wizard_and_boss() -> (Wizard, Boss) {
        (Wizard::new(100, 50), Boss::new(200, 10))
    }

    #[test]
    fn test_add() {
        let mut active = ActiveSpells::new();

        assert_eq!(active.effects.len(), 0);

        active.add_spell(Box::new(MagicMissile::new()));
        assert_eq!(active.effects.len(), 1);
        assert_eq!(active.effects[0].get_spell_type(), SpellType::MagicMissile);

        active.add_spell(Box::new(Poison::new()));
        assert_eq!(active.effects.len(), 2);
        assert_eq!(active.effects[0].get_spell_type(), SpellType::MagicMissile);
        assert_eq!(active.effects[1].get_spell_type(), SpellType::Poison);
    }

    #[test]
    #[should_panic]
    fn test_add_panic() {
        let mut active = ActiveSpells::new();

        assert_eq!(active.effects.len(), 0);

        active.add_spell(Box::new(Poison::new()));
        assert_eq!(active.effects.len(), 1);
        assert_eq!(active.effects[0].get_spell_type(), SpellType::Poison);

        // panic
        active.add_spell(Box::new(Poison::new()));
    }

    #[test]
    fn test_apply_effects() {
        let (mut wizard, mut boss) = create_wizard_and_boss();
        let mut active = ActiveSpells::new();

        assert_eq!(active.effects.len(), 0);

        active.add_spell(Box::new(MagicMissile::new()));
        active.add_spell(Box::new(Poison::new()));

        assert_eq!(active.effects.len(), 2);
        assert_eq!(active.effects[0].get_spell_type(), SpellType::MagicMissile);
        assert_eq!(active.effects[1].get_spell_type(), SpellType::Poison);

        let mut remaining_poison_turns = active.effects[1].get_remaining_turns();

        // 1st round, MagicMissile is removed (turns = 0), Poison decreased to 5
        active.apply_effects(&mut wizard, &mut boss);
        remaining_poison_turns -= 1;

        assert_eq!(active.effects.len(), 1);
        assert_eq!(active.effects[0].get_spell_type(), SpellType::Poison);
        assert_eq!(
            active.effects[0].get_remaining_turns(),
            remaining_poison_turns
        );

        // Repeat until remaining turns > 1
        while remaining_poison_turns > 1 {
            // Decrease counter
            remaining_poison_turns -= 1;

            // Apply effects and check if Poison is still in the effects with correct count
            active.apply_effects(&mut wizard, &mut boss);

            assert_eq!(active.effects.len(), 1);
            assert_eq!(active.effects[0].get_spell_type(), SpellType::Poison);
            assert_eq!(
                active.effects[0].get_remaining_turns(),
                remaining_poison_turns
            );
        }

        // Last turn
        assert_eq!(active.effects.len(), 1);
        assert_eq!(active.effects[0].get_spell_type(), SpellType::Poison);
        assert_eq!(active.effects[0].get_remaining_turns(), 1);

        active.apply_effects(&mut wizard, &mut boss);
        assert_eq!(active.effects.len(), 0);
    }

    #[test]
    fn test_get_allowed_spells_magic_missile() {
        let mut active = ActiveSpells::new();

        active.add_spell(Box::new(MagicMissile::new()));

        let allowed_spells = active.get_allowed_spells();

        assert_eq!(allowed_spells.len(), 5);
        assert_eq!(allowed_spells[0].get_spell_type(), SpellType::MagicMissile);
        assert_eq!(allowed_spells[1].get_spell_type(), SpellType::Drain);
        assert_eq!(allowed_spells[2].get_spell_type(), SpellType::Shield);
        assert_eq!(allowed_spells[3].get_spell_type(), SpellType::Poison);
        assert_eq!(allowed_spells[4].get_spell_type(), SpellType::Recharge);
    }

    #[test]
    fn test_get_allowed_spells_drain() {
        let mut active = ActiveSpells::new();

        active.add_spell(Box::new(Drain::new()));

        let allowed_spells = active.get_allowed_spells();

        assert_eq!(allowed_spells.len(), 5);
        assert_eq!(allowed_spells[0].get_spell_type(), SpellType::MagicMissile);
        assert_eq!(allowed_spells[1].get_spell_type(), SpellType::Drain);
        assert_eq!(allowed_spells[2].get_spell_type(), SpellType::Shield);
        assert_eq!(allowed_spells[3].get_spell_type(), SpellType::Poison);
        assert_eq!(allowed_spells[4].get_spell_type(), SpellType::Recharge);
    }

    #[test]
    fn test_get_allowed_spells_shield() {
        let mut wizard = Wizard::new(100, 100);
        let mut boss = Boss::new(100, 1);
        let mut active = ActiveSpells::new();

        active.add_spell(Box::new(Shield::new()));

        assert_eq!(active.effects.len(), 1);
        let mut remaining_turns = active.effects[0].get_remaining_turns();

        let allowed_spells = active.get_allowed_spells();

        assert_eq!(allowed_spells.len(), 4);
        assert_eq!(allowed_spells[0].get_spell_type(), SpellType::MagicMissile);
        assert_eq!(allowed_spells[1].get_spell_type(), SpellType::Drain);
        assert_eq!(allowed_spells[2].get_spell_type(), SpellType::Poison);
        assert_eq!(allowed_spells[3].get_spell_type(), SpellType::Recharge);

        // Check that Shield is again available when turns is 1 or 0
        while remaining_turns > 2 {
            active.effects[0].apply_round_effect(&mut wizard, &mut boss);
            remaining_turns -= 1;

            assert_eq!(active.effects[0].get_remaining_turns(), remaining_turns);

            let allowed_spells = active.get_allowed_spells();
            assert_eq!(allowed_spells.len(), 4);
            assert_eq!(allowed_spells[0].get_spell_type(), SpellType::MagicMissile);
            assert_eq!(allowed_spells[1].get_spell_type(), SpellType::Drain);
            assert_eq!(allowed_spells[2].get_spell_type(), SpellType::Poison);
            assert_eq!(allowed_spells[3].get_spell_type(), SpellType::Recharge);
        }

        // Now shield is again available (Turns 1)
        active.effects[0].apply_round_effect(&mut wizard, &mut boss);

        assert_eq!(active.effects[0].get_remaining_turns(), 1);

        let allowed_spells = active.get_allowed_spells();
        assert_eq!(allowed_spells.len(), 5);
        assert_eq!(allowed_spells[0].get_spell_type(), SpellType::MagicMissile);
        assert_eq!(allowed_spells[1].get_spell_type(), SpellType::Drain);
        assert_eq!(allowed_spells[2].get_spell_type(), SpellType::Shield);
        assert_eq!(allowed_spells[3].get_spell_type(), SpellType::Poison);
        assert_eq!(allowed_spells[4].get_spell_type(), SpellType::Recharge);

        // Now shield is again available (Turns 0)
        active.effects[0].apply_round_effect(&mut wizard, &mut boss);

        assert_eq!(active.effects[0].get_remaining_turns(), 0);

        let allowed_spells = active.get_allowed_spells();
        assert_eq!(allowed_spells.len(), 5);
        assert_eq!(allowed_spells[0].get_spell_type(), SpellType::MagicMissile);
        assert_eq!(allowed_spells[1].get_spell_type(), SpellType::Drain);
        assert_eq!(allowed_spells[2].get_spell_type(), SpellType::Shield);
        assert_eq!(allowed_spells[3].get_spell_type(), SpellType::Poison);
        assert_eq!(allowed_spells[4].get_spell_type(), SpellType::Recharge);
    }

    #[test]
    fn test_get_allowed_spells_poison() {
        let mut wizard = Wizard::new(100, 100);
        let mut boss = Boss::new(100, 1);
        let mut active = ActiveSpells::new();

        active.add_spell(Box::new(Poison::new()));

        assert_eq!(active.effects.len(), 1);
        let mut remaining_turns = active.effects[0].get_remaining_turns();

        let allowed_spells = active.get_allowed_spells();

        assert_eq!(allowed_spells.len(), 4);
        assert_eq!(allowed_spells[0].get_spell_type(), SpellType::MagicMissile);
        assert_eq!(allowed_spells[1].get_spell_type(), SpellType::Drain);
        assert_eq!(allowed_spells[2].get_spell_type(), SpellType::Shield);
        assert_eq!(allowed_spells[3].get_spell_type(), SpellType::Recharge);

        // Check that Shield is again available when turns is 1 or 0
        while remaining_turns > 2 {
            active.effects[0].apply_round_effect(&mut wizard, &mut boss);
            remaining_turns -= 1;

            assert_eq!(active.effects[0].get_remaining_turns(), remaining_turns);

            let allowed_spells = active.get_allowed_spells();
            assert_eq!(allowed_spells.len(), 4);
            assert_eq!(allowed_spells[0].get_spell_type(), SpellType::MagicMissile);
            assert_eq!(allowed_spells[1].get_spell_type(), SpellType::Drain);
            assert_eq!(allowed_spells[2].get_spell_type(), SpellType::Shield);
            assert_eq!(allowed_spells[3].get_spell_type(), SpellType::Recharge);
        }

        // Now shield is again available (Turns 1)
        active.effects[0].apply_round_effect(&mut wizard, &mut boss);

        assert_eq!(active.effects[0].get_remaining_turns(), 1);

        let allowed_spells = active.get_allowed_spells();
        assert_eq!(allowed_spells.len(), 5);
        assert_eq!(allowed_spells[0].get_spell_type(), SpellType::MagicMissile);
        assert_eq!(allowed_spells[1].get_spell_type(), SpellType::Drain);
        assert_eq!(allowed_spells[2].get_spell_type(), SpellType::Shield);
        assert_eq!(allowed_spells[3].get_spell_type(), SpellType::Poison);
        assert_eq!(allowed_spells[4].get_spell_type(), SpellType::Recharge);

        // Now shield is again available (Turns 0)
        active.effects[0].apply_round_effect(&mut wizard, &mut boss);

        assert_eq!(active.effects[0].get_remaining_turns(), 0);

        let allowed_spells = active.get_allowed_spells();
        assert_eq!(allowed_spells.len(), 5);
        assert_eq!(allowed_spells[0].get_spell_type(), SpellType::MagicMissile);
        assert_eq!(allowed_spells[1].get_spell_type(), SpellType::Drain);
        assert_eq!(allowed_spells[2].get_spell_type(), SpellType::Shield);
        assert_eq!(allowed_spells[3].get_spell_type(), SpellType::Poison);
        assert_eq!(allowed_spells[4].get_spell_type(), SpellType::Recharge);
    }

    #[test]
    fn test_get_allowed_spells_recharge() {
        let mut wizard = Wizard::new(100, 100);
        let mut boss = Boss::new(100, 1);
        let mut active = ActiveSpells::new();

        active.add_spell(Box::new(Recharge::new()));

        assert_eq!(active.effects.len(), 1);
        let mut remaining_turns = active.effects[0].get_remaining_turns();

        let allowed_spells = active.get_allowed_spells();

        assert_eq!(allowed_spells.len(), 4);
        assert_eq!(allowed_spells[0].get_spell_type(), SpellType::MagicMissile);
        assert_eq!(allowed_spells[1].get_spell_type(), SpellType::Drain);
        assert_eq!(allowed_spells[2].get_spell_type(), SpellType::Shield);
        assert_eq!(allowed_spells[3].get_spell_type(), SpellType::Poison);

        // Check that Shield is again available when turns is 1 or 0
        while remaining_turns > 2 {
            active.effects[0].apply_round_effect(&mut wizard, &mut boss);
            remaining_turns -= 1;

            assert_eq!(active.effects[0].get_remaining_turns(), remaining_turns);

            let allowed_spells = active.get_allowed_spells();
            assert_eq!(allowed_spells.len(), 4);
            assert_eq!(allowed_spells[0].get_spell_type(), SpellType::MagicMissile);
            assert_eq!(allowed_spells[1].get_spell_type(), SpellType::Drain);
            assert_eq!(allowed_spells[2].get_spell_type(), SpellType::Shield);
            assert_eq!(allowed_spells[3].get_spell_type(), SpellType::Poison);
        }

        // Now shield is again available (Turns 1)
        active.effects[0].apply_round_effect(&mut wizard, &mut boss);

        assert_eq!(active.effects[0].get_remaining_turns(), 1);

        let allowed_spells = active.get_allowed_spells();
        assert_eq!(allowed_spells.len(), 5);
        assert_eq!(allowed_spells[0].get_spell_type(), SpellType::MagicMissile);
        assert_eq!(allowed_spells[1].get_spell_type(), SpellType::Drain);
        assert_eq!(allowed_spells[2].get_spell_type(), SpellType::Shield);
        assert_eq!(allowed_spells[3].get_spell_type(), SpellType::Poison);
        assert_eq!(allowed_spells[4].get_spell_type(), SpellType::Recharge);

        // Now shield is again available (Turns 0)
        active.effects[0].apply_round_effect(&mut wizard, &mut boss);

        assert_eq!(active.effects[0].get_remaining_turns(), 0);

        let allowed_spells = active.get_allowed_spells();
        assert_eq!(allowed_spells.len(), 5);
        assert_eq!(allowed_spells[0].get_spell_type(), SpellType::MagicMissile);
        assert_eq!(allowed_spells[1].get_spell_type(), SpellType::Drain);
        assert_eq!(allowed_spells[2].get_spell_type(), SpellType::Shield);
        assert_eq!(allowed_spells[3].get_spell_type(), SpellType::Poison);
        assert_eq!(allowed_spells[4].get_spell_type(), SpellType::Recharge);
    }
}
