use crate::position::Position;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        self::Block::new(Position::new(0, 0), Orientation::Horizontal)
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}
