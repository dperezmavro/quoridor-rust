use crate::core::position::Position;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Tile {
    pub tile_type: TileType,
    pub state: TileState,
    pub position: Position,
}

impl Tile {
    pub fn new(
        tile_type: TileType,
        state: TileState,
        position: Position,
    ) -> Self {
        Tile {
            tile_type,
            state,
            position,
        }
    }

    pub fn occupy(&mut self) {
        self.state = TileState::Occupied;
    }

    pub fn free(&mut self) {
        self.state = TileState::Free
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum TileType {
    BlockPosition,
    #[default]
    Playable,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum TileState {
    Occupied,
    #[default]
    Free,
}
