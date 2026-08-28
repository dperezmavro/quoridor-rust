use crate::position::Position;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
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
}
