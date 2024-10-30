pub type DamageValue = u32;
pub type ArmorValue = u32;
pub type CostValue = u32;

pub trait Item {
    /// Description of the item
    fn name(&self) -> &str;

    /// Damage of the item
    fn damage(&self) -> DamageValue;

    /// Armor of the item
    fn armor(&self) -> ArmorValue;

    /// Cost of the item
    fn cost(&self) -> CostValue;
}
