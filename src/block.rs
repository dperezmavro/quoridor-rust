use crate::position::Position;

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct Block {
    pub orientation: Orientation,
    pub position: Position,
}

impl Block {
    pub fn new(orientation: Orientation, position: Position) -> Self {
        Self { orientation, position }
    }
}