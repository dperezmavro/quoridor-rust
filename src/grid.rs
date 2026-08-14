use crate::block::Orientation;
use crate::position::Position;
use std::cmp::PartialEq;

pub const GRID_SIZE: usize = 9;
const HORIZONTAL_BLOCK_MATRIX_Y: usize = GRID_SIZE - 1;
const HORIZONTAL_BLOCK_MATRIX_X: usize = GRID_SIZE - BLOCK_LENGTH;

const VERTICAL_BLOCK_MATRIX_Y: usize = GRID_SIZE - BLOCK_LENGTH;
const VERTICAL_BLOCK_MATRIX_X: usize = GRID_SIZE - 1;

const BLOCK_LENGTH: usize = 2;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    BlockPosition,
    Playable,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
        let mut playable_row: [Tile; GRID_SIZE] = [Tile::default(); GRID_SIZE];
        for i in 0..GRID_SIZE {
            playable_row[i] = Tile::new(
                TileType::Playable,
                TileState::Free,
                Position { y: 0, x: i },
            )
        }

        let player_1_tile = Tile::new(
            TileType::Playable,
            TileState::Occupied,
            Position {
                y: 0,
                x: GRID_SIZE / 2,
            },
        );
        let player_2_tile = Tile::new(
            TileType::Playable,
            TileState::Occupied,
            Position {
                y: GRID_SIZE - 1,
                x: GRID_SIZE / 2,
            },
        );
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

        let mut horizontal_blocks: Box<
            [[Tile; HORIZONTAL_BLOCK_MATRIX_X]; HORIZONTAL_BLOCK_MATRIX_Y],
        > = Box::new(
            [[Tile::default(); HORIZONTAL_BLOCK_MATRIX_X];
                HORIZONTAL_BLOCK_MATRIX_Y],
        );
        for y in 0..HORIZONTAL_BLOCK_MATRIX_Y {
            for x in 0..HORIZONTAL_BLOCK_MATRIX_X {
                horizontal_blocks[y][x] = Tile::new(
                    TileType::BlockPosition,
                    TileState::Free,
                    Position { y, x },
                );
            }
        }

        let mut vertical_blocks: Box<
            [[Tile; VERTICAL_BLOCK_MATRIX_X]; VERTICAL_BLOCK_MATRIX_Y],
        > = Box::new(
            [[Tile::default(); VERTICAL_BLOCK_MATRIX_X];
                VERTICAL_BLOCK_MATRIX_Y],
        );

        for y in 0..VERTICAL_BLOCK_MATRIX_Y {
            for x in 0..VERTICAL_BLOCK_MATRIX_X {
                vertical_blocks[y][x] = Tile::new(
                    TileType::BlockPosition,
                    TileState::Free,
                    Position { y, x },
                );
            }
        }

        Self {
            player_tiles,
            horizontal_blocks,
            vertical_blocks,
        }
    }

    pub fn get_reachable_neighbour_tiles(&self, p: Position) -> Vec<Tile> {
        assert_eq!(true, p.x < GRID_SIZE && p.y < GRID_SIZE);

        let mut available_blocks: Vec<Tile> = Vec::new();
        //top
        if p.y > 0 {
            if self.horizontal_blocks[p.y - 1][p.x].state == TileState::Free {
                available_blocks.push(self.player_tiles[p.y - 1][p.x]);
            }
        }
        //bottom
        if p.y < GRID_SIZE - 1 {
            if self.horizontal_blocks[p.y + 1][p.x].state == TileState::Free {
                available_blocks.push(self.player_tiles[p.y + 1][p.x]);
            }
        }
        //right
        if p.x < GRID_SIZE - 1 {
            if self.vertical_blocks[p.y][p.x + 1].state == TileState::Free {
                available_blocks.push(self.player_tiles[p.y][p.x + 1]);
            }
        }
        //left
        if p.x > 0 {
            if self.vertical_blocks[p.y][p.x - 1].state == TileState::Free {
                available_blocks.push(self.player_tiles[p.y][p.x - 1]);
            }
        }

        available_blocks
    }

    // pub fn is_grid_traversable(&self) -> bool {
    //     let mut reachable_tiles: Vec<Tile> = vec![Tile::new(
    //         TileType::Playable,
    //         TileState::Free,
    //         Position { x: 0, y: 0 },
    //     )];
    //     let mut visited_tiles: HashSet<Tile> = HashSet::new();
    //     while let tile = reachable_tiles.iter().next() {
    //         match tile {
    //             Some(t) => {
    //                 if !visited_tiles.contains(t) {
    //                     visited_tiles.insert(*t);
    //                     reachable_tiles.push(*t);
    //                     // TODO get neighbours of tiles and traverse as well
    //                 }
    //             }
    //             None => {}
    //         }
    //     }
    //
    //     false
    // }

    pub fn place_block(
        &mut self,
        p: Position,
        orientation: Orientation,
    ) -> bool {
        assert_eq!(
            true,
            p.x <= GRID_SIZE - BLOCK_LENGTH && p.y <= GRID_SIZE - BLOCK_LENGTH
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
        grid.horizontal_blocks[0][0] = Tile::new(
            TileType::BlockPosition,
            TileState::Occupied,
            Position { x: 0, y: 0 },
        );
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
    #[test]
    fn get_reachable_neighbour_tiles() {
        // let grid = Grid::new();
        // let result = grid.get_reachable_neighbour_tiles(Position::new(0, 0));
        // assert_eq!(result.len(), 2);
        //
        // let result = grid.get_reachable_neighbour_tiles(Position::new(1, 1));
        // assert_eq!(result.len(), 4);
        //
        // let result = grid.get_reachable_neighbour_tiles(Position::new(
        //     GRID_SIZE - 1,
        //     GRID_SIZE - 1,
        // ));
        // assert_eq!(result.len(), 2);

        // let result = grid.get_reachable_neighbour_tiles(Position::new(
        //     GRID_SIZE - 2,
        //     GRID_SIZE - 1,
        // ));
        // assert_eq!(result.len(), 3);
        //
        // let result = grid.get_reachable_neighbour_tiles(Position::new(
        //     GRID_SIZE - 1,
        //     GRID_SIZE - 2,
        // ));
        // assert_eq!(result.len(), 3);
        //
        // let mut g = Grid::new();
        // let result = grid.get_reachable_neighbour_tiles(Position::new(0, 0));
        // assert_eq!(result.len(), 2);
        // g.place_block(Position::new(0, 0), Orientation::Horizontal);
        // let result = grid.get_reachable_neighbour_tiles(Position::new(0, 0));
        // assert_eq!(result.len(), 1);
    }
}
