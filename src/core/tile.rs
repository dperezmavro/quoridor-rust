use crate::core::position::Position;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Tile {
    pub state: TileState,
    pub position: Position,
}

impl Tile {
    pub fn new(
        state: TileState,
        position: Position,
    ) -> Self {
        Tile {
            state,
            position,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum TileState {
    Occupied,
    #[default]
    Free,
}
