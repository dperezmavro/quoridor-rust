use crate::core::position::Position;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Tile {
    pub position: Position,
}

impl Tile {
    pub fn new(position: Position) -> Self {
        Tile { position }
    }
}
