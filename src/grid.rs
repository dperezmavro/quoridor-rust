use crate::grid::TileType::Playable;

const NUM_PLAYABLE_ROWS: usize = 9;
const NUM_VERTICAL_BLOCK_SLOTS: usize = 8;

const GRID_SIZE: usize = NUM_VERTICAL_BLOCK_SLOTS + NUM_PLAYABLE_ROWS;

pub struct Grid {
    pub size: u32,
    tiles: Box<[[Tile; GRID_SIZE]; GRID_SIZE]>,
}

#[derive(Clone)]
pub struct Tile {
    tile_type: TileType,
    state: TileState,
}

impl Tile {
    pub fn new(tile_type: TileType, state: TileState) -> Self {
        Tile { tile_type, state }
    }

    pub fn get_dimensions(&self) -> usize {
        match self.tile_type {
            TileType::BlockNode => 2,
            TileType::Playable => 4,
            TileType::BlockPosition => 4,
        }
    }
}

#[derive(Clone)]

pub enum TileType {
    BlockPosition,
    BlockNode,
    Playable,
}

#[derive(Clone)]

pub enum TileState {
    PlayerOccupied { player_id: String },
    BlockOccupied { player_id: String },
    Free,
}

impl Grid {
    pub fn new(size: u32) -> Self {
        let playable_row: [Tile; GRID_SIZE] = [
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::Playable, TileState::Free),
        ];

        let horizontal_block_row: [Tile; GRID_SIZE] = [
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::BlockNode, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::BlockNode, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::BlockNode, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::BlockNode, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::BlockNode, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::BlockNode, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::BlockNode, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
            Tile::new(TileType::BlockNode, TileState::Free),
            Tile::new(TileType::BlockPosition, TileState::Free),
        ];

        let tiles: Box<[[Tile; GRID_SIZE]; GRID_SIZE]> = Box::new([
            playable_row.clone(),
            horizontal_block_row.clone(),
            playable_row.clone(),
            horizontal_block_row.clone(),
            playable_row.clone(),
            horizontal_block_row.clone(),
            playable_row.clone(),
            horizontal_block_row.clone(),
            playable_row.clone(),
            horizontal_block_row.clone(),
            playable_row.clone(),
            horizontal_block_row.clone(),
            playable_row.clone(),
            horizontal_block_row.clone(),
            playable_row.clone(),
            horizontal_block_row.clone(),
            playable_row.clone(),
        ]);

        Self { size, tiles }
    }
}
