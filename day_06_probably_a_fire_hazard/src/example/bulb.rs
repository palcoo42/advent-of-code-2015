pub trait Bulb: Default {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn toggle(&mut self);
    fn is_on(&self) -> bool;
    fn brightness(&self) -> usize;
}
