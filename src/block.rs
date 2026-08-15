use crate::position::Position;

#[derive(Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy)]
pub struct Block {
    pub orientation: Orientation,
    pub position: Position,
    pub is_placed: bool,
}

impl Block {
    pub fn new(position: Position, orientation: Orientation) -> Self {
        Self {
            orientation,
            position,
            is_placed: false,
        }
    }

    pub fn default() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            position: Position::new(0, 0),
            is_placed: false,
        }
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}
