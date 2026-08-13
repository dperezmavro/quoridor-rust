use crate::block::{Block, Orientation};
use crate::position::Position;
use std::cmp::PartialEq;

pub const GRID_SIZE: usize = 9;
const HORIZONTAL_BLOCK_MATRIX_Y: usize = 4;
const HORIZONTAL_BLOCK_MATRIX_X: usize = 5;

const VERTICAL_BLOCK_MATRIX_Y: usize = 5;
const VERTICAL_BLOCK_MATRIX_X: usize = 4;

const BLOCK_LENGTH: usize = 2;

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

#[derive(Clone, Copy, PartialEq)]
pub enum TileState {
    Occupied,
    Free,
}

pub struct Grid {
    pub player_tiles: Box<[[Tile; GRID_SIZE]; GRID_SIZE]>,
    pub horizontal_blocks:
        Box<[[Tile; HORIZONTAL_BLOCK_MATRIX_X]; HORIZONTAL_BLOCK_MATRIX_Y]>,
    pub vertical_blocks:
        Box<[[Tile; VERTICAL_BLOCK_MATRIX_X]; VERTICAL_BLOCK_MATRIX_Y]>,
}

impl Grid {
    pub fn new() -> Self {
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

    pub fn get_available_tiles(&self, p: Position) -> Vec<Tile> {
        assert_eq!(
            true,
            p.x >= 0 && p.y >= 0 && p.x < GRID_SIZE && p.y < GRID_SIZE
        );

        let mut available_blocks: Vec<Tile> = Vec::new();
        //top
        if p.x > 0 {
            // do something about y
            if self.horizontal_blocks[p.y - 1][p.x].state == TileState::Free {
                available_blocks.push(self.horizontal_blocks[p.y - 1][p.x]);
            }
        }
        //bottom
        if p.x < GRID_SIZE - 1 {
            // do something about y
        }
        //right
        if p.y < GRID_SIZE - 1 {
            // do something about x
        }
        //left
        if p.y > 0 {
            // todo do something about X
        }

        available_blocks
    }

    pub fn place_block(
        &mut self,
        p: Position,
        orientation: Orientation,
    ) -> bool {
        assert_eq!(
            true,
            p.x >= 0
                && p.x <= GRID_SIZE - BLOCK_LENGTH
                && p.y >= 0
                && p.y <= GRID_SIZE - BLOCK_LENGTH
        );

        match orientation {
            Orientation::Horizontal => {
                if self.horizontal_blocks[p.y][p.x].state == TileState::Free
                    && self.horizontal_blocks[p.y][p.x + 1].state
                        == TileState::Free
                {
                    self.horizontal_blocks[p.y][p.x].state =
                        TileState::Occupied;
                    self.horizontal_blocks[p.y][p.x + 1].state =
                        TileState::Occupied;

                    // TODO(dio): add a check here for the validity of the block placement
                    // can't be allowed if it divides the board in two
                    return true;
                }
            }
            Orientation::Vertical => {
                if self.horizontal_blocks[p.y][p.x].state == TileState::Free
                    && self.horizontal_blocks[p.y + 1][p.x].state
                        == TileState::Free
                {
                    self.horizontal_blocks[p.y][p.x].state =
                        TileState::Occupied;
                    self.horizontal_blocks[p.y + 1][p.x].state =
                        TileState::Occupied;

                    // TODO(dio): add a check here for the validity of the block placement
                    // can't be allowed if it divides the board in two
                    return true;
                }
            }
        };

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_available_tiles() {
        let mut grid = Grid::new();
        grid.horizontal_blocks[0][0] =
            Tile::new(TileType::BlockPosition, TileState::Occupied);
    }

    #[test]
    #[should_panic]
    fn test_place_block_illegal_horizontal() {
        let mut grid = Grid::new();
        let success =
            grid.place_block(Position::new(8, 0), Orientation::Horizontal);
        assert_eq!(success, false);
    }

    #[test]
    #[should_panic]
    fn test_place_block_illegal_vertical() {
        let mut grid = Grid::new();
        let success =
            grid.place_block(Position::new(0, 8), Orientation::Vertical);
        assert_eq!(success, false);
    }

    #[test]
    fn place_block() {
        let mut grid = Grid::new();
        let result =
            grid.place_block(Position::new(0, 0), Orientation::Horizontal);
        assert_eq!(result, true);

        let result =
            grid.place_block(Position::new(1, 0), Orientation::Horizontal);
        assert_eq!(result, false);

        let result =
            grid.place_block(Position::new(3, 0), Orientation::Vertical);
        assert_eq!(result, true);

        // attempt to place block on occupied position
        let result =
            grid.place_block(Position::new(3, 0), Orientation::Vertical);
        assert_eq!(result, false);
    }
}
