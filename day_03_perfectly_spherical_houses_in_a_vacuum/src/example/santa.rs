use super::direction::Direction;

#[derive(Clone, Default, Debug)]
pub struct Santa {
    pub x: i32,
    pub y: i32,
}

impl Santa {
    pub fn move_to(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
            Direction::Down => {
                self.y -= 1;
            }
            Direction::Up => {
                self.y += 1;
            }
        }
    }
}
