use super::item::{ArmorValue, CostValue, DamageValue, Item};

#[derive(Debug, Clone)]
pub struct Ring {
    name: String,
    damage: DamageValue,
    armor: ArmorValue,
    cost: CostValue,
}

impl Ring {
    pub fn new(name: &str, damage: DamageValue, armor: ArmorValue, cost: CostValue) -> Self {
        Self {
            name: name.to_string(),
            damage,
            armor,
            cost,
        }
    }
}

impl Item for Ring {
    fn name(&self) -> &str {
        &self.name
    }

    fn damage(&self) -> DamageValue {
        self.damage
    }

    fn armor(&self) -> ArmorValue {
        self.armor
    }

    fn cost(&self) -> CostValue {
        self.cost
    }
}
