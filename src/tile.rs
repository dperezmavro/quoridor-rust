use crate::position::Position;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
    pub fn default() -> Self {
        Self {
            state: TileState::Free,
            tile_type: TileType::Playable,
            position: Position { x: 0, y: 0 },
        }
    }

    pub fn occupy(&mut self) {
        self.state = TileState::Occupied;
    }

    pub fn is_free(&self) -> bool {
        self.state == TileState::Free
    }

    pub fn free(&mut self) {
        self.state = TileState::Free
    }

    pub fn get_print_char(&self) -> char {
        match self.tile_type {
            TileType::BlockPosition => 'b',
            TileType::Playable => match self.state {
                TileState::Free => ' ',
                TileState::Occupied => 'P',
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TileType {
    BlockPosition,
    Playable,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TileState {
    Occupied,
    Free,
}
