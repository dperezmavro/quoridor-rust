use crate::block::Orientation;
use crate::position::Position;
use std::cmp::PartialEq;
use std::collections::HashSet;

pub const GRID_SIZE: usize = 9;
const HORIZONTAL_BLOCK_MATRIX_Y: usize = GRID_SIZE - 1;
const HORIZONTAL_BLOCK_MATRIX_X: usize = GRID_SIZE;

const VERTICAL_BLOCK_MATRIX_Y: usize = GRID_SIZE;
const VERTICAL_BLOCK_MATRIX_X: usize = GRID_SIZE - 1;

const BLOCK_LENGTH: usize = 2;

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

pub struct Grid {
    pub player_tiles: Box<[Box<[Tile; GRID_SIZE]>; GRID_SIZE]>,
    pub horizontal_blocks:
        Box<[[Tile; HORIZONTAL_BLOCK_MATRIX_X]; HORIZONTAL_BLOCK_MATRIX_Y]>,
    pub vertical_blocks:
        Box<[[Tile; VERTICAL_BLOCK_MATRIX_X]; VERTICAL_BLOCK_MATRIX_Y]>,
}

fn generate_row(y: usize) -> Box<[Tile; GRID_SIZE]> {
    let mut res = Box::new([Tile::default(); GRID_SIZE]);
    let default = Tile::default();
    for i in 0..GRID_SIZE {
        res[i] = Tile {
            position: Position { x: i, y },
            ..default
        };
    }

    res
}

impl Grid {
    pub fn new() -> Self {
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
        let mut first_row = generate_row(0);
        first_row[GRID_SIZE / 2] = player_1_tile.clone();
        let mut last_row = generate_row(GRID_SIZE - 1);
        last_row[GRID_SIZE / 2] = player_2_tile.clone();

        let player_tiles: Box<[Box<[Tile; GRID_SIZE]>; GRID_SIZE]> =
            Box::new([
                first_row,
                generate_row(1),
                generate_row(2),
                generate_row(3),
                generate_row(4),
                generate_row(5),
                generate_row(6),
                generate_row(7),
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

    pub fn get_reachable_neighbour_tiles(&self, p: Position) -> Vec<&Tile> {
        assert_eq!(true, p.x < GRID_SIZE && p.y < GRID_SIZE);

        let mut available_tiles: Vec<&Tile> = Vec::new();
        //top
        if p.y > 0 {
            if self.horizontal_blocks[p.y - 1][p.x].state == TileState::Free {
                available_tiles.push(&self.player_tiles[p.y - 1][p.x]);
            }
        }
        //bottom
        if p.y < GRID_SIZE - 1 {
            if self.horizontal_blocks[p.y][p.x].state == TileState::Free {
                available_tiles.push(&self.player_tiles[p.y + 1][p.x]);
            }
        }

        // right
        if p.x < GRID_SIZE - 1 {
            if self.vertical_blocks[p.y][p.x].state == TileState::Free {
                available_tiles.push(&self.player_tiles[p.y][p.x + 1]);
            }
        }
        //left
        if p.x > 0 {
            if self.vertical_blocks[p.y][p.x - 1].state == TileState::Free {
                available_tiles.push(&self.player_tiles[p.y][p.x - 1]);
            }
        }

        available_tiles
    }

    pub fn is_grid_traversible(&self) -> bool {
        let mut left_to_explore: HashSet<&Tile> = HashSet::new();
        left_to_explore.insert(&Tile {
            position: Position { x: 0, y: 0 },
            state: TileState::Free,
            tile_type: TileType::Playable,
        });
        let mut visited: HashSet<&Tile> = HashSet::new();
        while left_to_explore.len() > 0 {
            let t_v = Vec::from_iter(left_to_explore.iter().cloned());
            let t = t_v[0];
            println!("[+] Exploring tile {:?}", t.position);
            visited.insert(t);
            let neighbours = self.get_reachable_neighbour_tiles(t.position);
            for tt in neighbours {
                println!(
                    "[*] Neighbour for tile {:?}: {:?}",
                    t.position, tt.position
                );

                if !visited.contains(&tt) {
                    left_to_explore.insert(tt);
                }
            }
            println!("[-] Removing  tile {:?}", t.position);
            left_to_explore.remove(t);
        }

        visited.len() == GRID_SIZE * GRID_SIZE
    }

    pub fn place_block(
        &mut self,
        p: Position,
        orientation: Orientation,
    ) -> bool {
        assert_eq!(true, p.x < GRID_SIZE - 1 && p.y < GRID_SIZE - 1);

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
        struct TestCase(usize, usize, Orientation, bool);

        let tests = vec![
            TestCase(0, 0, Orientation::Horizontal, true),
            TestCase(1, 0, Orientation::Horizontal, false),
            TestCase(3, 0, Orientation::Horizontal, true),
            TestCase(3, 0, Orientation::Horizontal, false),
        ];

        let mut grid = Grid::new();

        tests.iter().for_each(|t| {
            let result =
                grid.place_block(Position { x: t.0, y: t.1 }, t.2.clone());
            assert_eq!(t.3, result);
        });
    }
    #[test]
    fn get_reachable_neighbour_tiles() {
        struct TestCase(usize, usize, usize);
        let tests = vec![
            TestCase(0, 0, 2),
            TestCase(1, 1, 4),
            TestCase(GRID_SIZE - 1, GRID_SIZE - 1, 2),
            TestCase(0, GRID_SIZE - 1, 2),
            TestCase(GRID_SIZE - 1, 0, 2),
            TestCase(GRID_SIZE - 3, GRID_SIZE - 3, 4),
            TestCase(GRID_SIZE - 2, 0, 3),
            TestCase(0, GRID_SIZE - 2, 3),
            TestCase(GRID_SIZE - 1, GRID_SIZE - 2, 3),
            TestCase(GRID_SIZE - 2, GRID_SIZE - 1, 3),
        ];

        tests.iter().for_each(|t| {
            let grid = Grid::new();
            let result =
                grid.get_reachable_neighbour_tiles(Position { x: t.0, y: t.1 });
            assert_eq!(t.2, result.len());
        });
    }
    #[test]
    fn get_reachable_neighbour_tiles_with_blocks() {
        // place horizontal block
        let mut g = Grid::new();
        let result = g.get_reachable_neighbour_tiles(Position::new(0, 0));
        assert_eq!(result.len(), 2);
        let placed =
            g.place_block(Position::new(0, 0), Orientation::Horizontal);
        assert_eq!(placed, true);
        let result = g.get_reachable_neighbour_tiles(Position::new(0, 0));
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].position, Position::new(1, 0));

        // place vertical block
        let mut g = Grid::new();
        let placed = g.place_block(Position::new(0, 0), Orientation::Vertical);
        assert_eq!(placed, true);
        let result = g.get_reachable_neighbour_tiles(Position::new(0, 0));
        assert_eq!(result.len(), 1);
        // assert_eq!(result[0].position, Position::new(0, 1));
    }

    #[test]
    fn get_reachable_neighbour_tiles_assert() {
        let g = Grid::new();
        let result = g.get_reachable_neighbour_tiles(Position::new(0, 0));
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].position.x, 0);
        assert_eq!(result[0].position.y, 1);

        assert_eq!(result[1].position.x, 1);
        assert_eq!(result[1].position.y, 0);
    }

    #[test]
    fn is_grid_traversable() {
        // empty board
        let g = Grid::new();
        let result = g.is_grid_traversible();
        assert_eq!(result, true);

        // single block
        let mut g = Grid::new();
        g.place_block(Position::new(0, 1), Orientation::Horizontal);
        let result = g.is_grid_traversible();
        assert_eq!(result, true);
    }

    // #[test]
    // fn is_grid_traversable_illegal_blocks() {
    //     // illegal block placement, but
    //     let mut g = Grid::new();
    //     let res = g.place_block(Position::new(0, 1), Orientation::Horizontal);
    //     assert_eq!(res, true);
    //     let res = g.place_block(Position::new(1, 0), Orientation::Horizontal);
    //     assert_eq!(res, true);
    //     let result = g.is_grid_traversible();
    //     assert_eq!(result, false);
    // }
}
