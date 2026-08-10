use crate::grid::TileType::Playable;

const NUM_PLAYABLE_ROWS: usize = 9;
const NUM_VERTICAL_BLOCK_SLOTS: usize = 8;

const GRID_SIZE: usize = NUM_VERTICAL_BLOCK_SLOTS + NUM_PLAYABLE_ROWS;

pub struct Grid {
    pub size: u32,
    tiles: Option<Box<[[Tile; GRID_SIZE]; GRID_SIZE]>>,
}

pub struct Tile {
    Type: TileType,
    State: PlayableTileState,
}

pub enum TileType {
    BlockPosition,
    BlockNode,
    Playable(PlayableTileState),
}

pub enum PlayableTileState {
    PlayerOccupied { player_id: String },
    BlockOccupied { player_id: String },
    Free,
}

impl Grid {
    pub fn new(size: u32) -> Self {
        // let t = [[Tile {
        //     State: PlayableTileState::Free,
        //     Type: TileType::BlockNode,
        // }; GRID_SIZE]; GRID_SIZE];
        Self { size, tiles: None }
    }
}
