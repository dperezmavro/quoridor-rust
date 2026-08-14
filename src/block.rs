use crate::position::Position;

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct Block {
    pub orientation: Orientation,
    pub position: Option<Position>,
    pub is_placed: bool,
}

impl Block {
    pub fn new() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            position: None,
            is_placed: false,
        }
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = Some(position);
    }
}
