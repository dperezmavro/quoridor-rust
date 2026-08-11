use crate::block::Block;

pub const GRID_SIZE: usize = 9;
const HORIZONTAL_BLOCK_MATRIX_Y: usize = 4;
const HORIZONTAL_BLOCK_MATRIX_X: usize = 5;

const VERTICAL_BLOCK_MATRIX_Y: usize = 5;
const VERTICAL_BLOCK_MATRIX_X: usize = 4;

pub struct Grid {
    pub player_tiles: Box<[[Tile; GRID_SIZE]; GRID_SIZE]>,
    pub horizontal_blocks:
        Box<[[Tile; HORIZONTAL_BLOCK_MATRIX_X]; HORIZONTAL_BLOCK_MATRIX_Y]>,
    pub vertical_blocks:
        Box<[[Tile; VERTICAL_BLOCK_MATRIX_X]; VERTICAL_BLOCK_MATRIX_Y]>,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub state: TileState,
}

impl Tile {
    pub fn new(tile_type: TileType, state: TileState) -> Self {
        Tile { tile_type, state }
    }

    pub fn get_dimensions(&self) -> usize {
        match self.tile_type {
            TileType::Playable => 4,
            TileType::BlockPosition => 4,
        }
    }
}

#[derive(Clone, Copy)]
pub enum TileType {
    BlockPosition,
    Playable,
}

#[derive(Clone, Copy)]
pub enum TileState {
    Occupied,
    Free,
}

impl Grid {
    pub fn new(player_1_id: String, player_2_id: String) -> Self {
        let playable_row: [Tile; GRID_SIZE] = [
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
        ];

        let player_1_tile = Tile::new(TileType::Playable, TileState::Occupied);
        let player_2_tile = Tile::new(TileType::Playable, TileState::Occupied);
        let mut first_row = playable_row.clone();
        first_row[GRID_SIZE / 2] = player_1_tile.clone();
        let mut last_row = playable_row.clone();
        last_row[GRID_SIZE / 2] = player_2_tile.clone();

        let player_tiles: Box<[[Tile; GRID_SIZE]; GRID_SIZE]> = Box::new([
            first_row,
            playable_row.clone(),
            playable_row.clone(),
            playable_row.clone(),
            playable_row.clone(),
            playable_row.clone(),
            playable_row.clone(),
            playable_row.clone(),
            last_row,
        ]);

        let horizontal_blocks = Box::new(
            [[Tile::new(TileType::BlockPosition, TileState::Free);
                HORIZONTAL_BLOCK_MATRIX_X];
                HORIZONTAL_BLOCK_MATRIX_Y],
        );

        let vertical_blocks = Box::new(
            [[Tile::new(TileType::BlockPosition, TileState::Free);
                VERTICAL_BLOCK_MATRIX_X]; VERTICAL_BLOCK_MATRIX_Y],
        );

        Self {
            player_tiles,
            horizontal_blocks,
            vertical_blocks,
        }
    }
}
