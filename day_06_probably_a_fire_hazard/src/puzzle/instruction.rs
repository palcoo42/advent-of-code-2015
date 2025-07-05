use puzzler::grids::point::Point;

use crate::puzzle::action::Action;

#[derive(Debug)]
pub struct Instruction {
    pub action: Action,
    pub from: Point,
    pub to: Point,
}
