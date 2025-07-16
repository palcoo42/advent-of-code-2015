use crate::puzzle::item::Item;

#[derive(Debug)]
pub struct Equipment<'a> {
    pub weapons: Vec<Vec<&'a Item>>,
    pub armors: Vec<Vec<&'a Item>>,
    pub rings: Vec<Vec<&'a Item>>,
}
