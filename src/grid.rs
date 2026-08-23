use crate::block::{Block, Orientation};
use crate::player::{Player, StartingPosition};
use crate::position::Position;
use crate::tile::{Tile, TileState, TileType};
use std::collections::HashSet;

pub const GRID_SIZE: usize = 9;
const HORIZONTAL_BLOCK_MATRIX_Y: usize = GRID_SIZE - 1;
const HORIZONTAL_BLOCK_MATRIX_X: usize = GRID_SIZE;

const VERTICAL_BLOCK_MATRIX_Y: usize = GRID_SIZE;
const VERTICAL_BLOCK_MATRIX_X: usize = GRID_SIZE - 1;

#[derive(Debug)]
pub struct Grid {
    pub player_tiles: Box<[Box<[Tile; GRID_SIZE]>; GRID_SIZE]>,
    horizontal_blocks:
        Box<[[Tile; HORIZONTAL_BLOCK_MATRIX_X]; HORIZONTAL_BLOCK_MATRIX_Y]>,
    vertical_blocks:
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
        first_row[GRID_SIZE / 2] = player_1_tile;
        let mut last_row = generate_row(GRID_SIZE - 1);
        last_row[GRID_SIZE / 2] = player_2_tile;

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
        assert!(p.x < GRID_SIZE && p.y < GRID_SIZE);

        let mut available_tiles: Vec<&Tile> = Vec::new();
        //top
        if p.y > 0 && self.horizontal_blocks[p.y - 1][p.x].is_free() {
            available_tiles.push(&self.player_tiles[p.y - 1][p.x]);
        }
        //bottom
        if p.y < GRID_SIZE - 1 && self.horizontal_blocks[p.y][p.x].is_free() {
            available_tiles.push(&self.player_tiles[p.y + 1][p.x]);
        }

        //left
        if p.x > 0 && self.vertical_blocks[p.y][p.x - 1].is_free() {
            available_tiles.push(&self.player_tiles[p.y][p.x - 1]);
        }
        // right
        if p.x < GRID_SIZE - 1 && self.vertical_blocks[p.y][p.x].is_free() {
            available_tiles.push(&self.player_tiles[p.y][p.x + 1]);
        }

        available_tiles
    }

    pub fn can_player_reach_goal(&self, p: &Player) -> bool {
        let goal_y = match p.starting_position {
            StartingPosition::Top => GRID_SIZE - 1,
            StartingPosition::Bottom => 0,
        };

        let mut visited: HashSet<&Tile> = HashSet::new();
        let mut tiles_for_exploring: HashSet<&Tile> = HashSet::new();

        tiles_for_exploring
            .insert(&self.player_tiles[p.position.y][p.position.x]);

        while !tiles_for_exploring.is_empty() {
            // let neighbours = self.get_reachable_neighbour_tiles(p.position);
            let curr_tile = tiles_for_exploring.iter().cloned().next().unwrap();
            if visited.contains(curr_tile) {
                continue;
            }

            visited.insert(curr_tile);
            let neighbors =
                self.get_reachable_neighbour_tiles(curr_tile.position);

            for t in neighbors {
                if t.position.y == goal_y {
                    return true;
                }

                if !visited.contains(&t) {
                    tiles_for_exploring.insert(t);
                }
            }

            tiles_for_exploring.remove(curr_tile);
        }

        false
    }
    pub fn is_grid_traversable(&self) -> bool {
        let mut left_to_explore: HashSet<&Tile> = HashSet::new();
        left_to_explore.insert(&self.player_tiles[0][0]);
        let mut visited: HashSet<&Tile> = HashSet::new();
        while !left_to_explore.is_empty() {
            let t_v = Vec::from_iter(left_to_explore.iter().cloned());
            let t = t_v[0];
            visited.insert(t);
            let neighbours = self.get_reachable_neighbour_tiles(t.position);
            for tt in neighbours {
                if !visited.contains(&tt) {
                    left_to_explore.insert(tt);
                }
            }
            left_to_explore.remove(t);
        }

        visited.len() == GRID_SIZE * GRID_SIZE
    }

    pub fn place_block(&mut self, b: &Block) -> bool {
        let orientation = b.orientation;
        let p = b.position;

        assert!(p.x < GRID_SIZE - 1 && p.y < GRID_SIZE - 1);

        match orientation {
            Orientation::Horizontal => {
                if self.horizontal_blocks[p.y][p.x].is_free()
                    && self.horizontal_blocks[p.y][p.x + 1].is_free()
                {
                    self.horizontal_blocks[p.y][p.x].occupy();
                    self.horizontal_blocks[p.y][p.x + 1].occupy();

                    if !self.is_grid_traversable() {
                        self.horizontal_blocks[p.y][p.x].free();
                        self.horizontal_blocks[p.y][p.x + 1].free();
                        return false;
                    }

                    return true;
                }
            }
            Orientation::Vertical => {
                if self.vertical_blocks[p.y][p.x].is_free()
                    && self.vertical_blocks[p.y + 1][p.x].is_free()
                {
                    self.vertical_blocks[p.y][p.x].occupy();
                    self.vertical_blocks[p.y + 1][p.x].occupy();

                    if !self.is_grid_traversable() {
                        self.vertical_blocks[p.y][p.x].free();
                        self.vertical_blocks[p.y + 1][p.x].free();
                        return false;
                    }

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
    use crate::block::Block;
    use crate::player::Player;

    #[test]
    fn test_get_available_tiles() {
        let mut grid = Grid::new();
        grid.horizontal_blocks[0][0] = Tile::new(
            TileType::BlockPosition,
            TileState::Occupied,
            Position { x: 0, y: 0 },
        );
        assert!(grid.is_grid_traversable());
    }

    #[test]
    #[should_panic]
    fn test_place_block_illegal() {
        struct TestCase(Block);
        let tests = [
            TestCase(Block::new(Position::new(8, 0), Orientation::Horizontal)),
            TestCase(Block::new(Position::new(0, 8), Orientation::Horizontal)),
        ];
        for t in tests.iter() {
            let mut grid = Grid::new();
            let success = grid.place_block(&t.0);
            assert!(!success);
        }
    }

    #[test]
    fn place_block_explicit() {
        struct TestCase(usize, usize, Orientation, bool);

        let tests = [
            TestCase(0, 0, Orientation::Horizontal, true),
            TestCase(1, 0, Orientation::Horizontal, false),
            TestCase(3, 0, Orientation::Horizontal, true),
            TestCase(3, 0, Orientation::Horizontal, false),
        ];

        let mut grid = Grid::new();

        tests.iter().for_each(|t| {
            let result =
                grid.place_block(&Block::new(Position { x: t.0, y: t.1 }, t.2));
            assert_eq!(t.3, result);
        });
    }

    #[test]
    fn test_place_blocks_bulk() {
        struct TestCase(Vec<Block>, bool);

        let tests = [
            TestCase(
                vec![
                    Block::new(Position::new(0, 3), Orientation::Horizontal),
                    Block::new(Position::new(2, 3), Orientation::Horizontal),
                    Block::new(Position::new(4, 3), Orientation::Horizontal),
                    Block::new(Position::new(6, 3), Orientation::Horizontal),
                    Block::new(Position::new(7, 5), Orientation::Horizontal),
                ],
                true,
            ),
            TestCase(
                vec![
                    Block::new(Position::new(0, 3), Orientation::Horizontal),
                    Block::new(Position::new(2, 3), Orientation::Horizontal),
                    Block::new(Position::new(4, 3), Orientation::Horizontal),
                    Block::new(Position::new(6, 3), Orientation::Horizontal),
                    Block::new(Position::new(7, 5), Orientation::Horizontal),
                    Block::new(Position::new(6, 4), Orientation::Vertical),
                ],
                false,
            ),
        ];

        tests.iter().for_each(|t| {
            let mut g = Grid::new();
            let mut result: bool = true;
            for b in t.0.iter().cloned() {
                result &= g.place_block(&b);
            }
            assert_eq!(t.1, result);
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
        let placed = g.place_block(&Block::new(
            Position::new(0, 0),
            Orientation::Horizontal,
        ));
        assert!(placed);
        let result = g.get_reachable_neighbour_tiles(Position::new(0, 0));
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].position, Position::new(1, 0));

        // place vertical block
        let mut g = Grid::new();
        let placed = g.place_block(&Block::new(
            Position::new(0, 0),
            Orientation::Vertical,
        ));
        assert!(placed);
        let result = g.get_reachable_neighbour_tiles(Position::new(0, 0));
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].position, Position::new(0, 1));
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
        struct TestCase(Vec<Block>, bool);

        let tests = [
            TestCase(vec![], true),
            TestCase(vec![Block::default()], true),
        ];

        tests.iter().for_each(|t| {
            let mut g = Grid::new();
            t.0.iter().for_each(|block| {
                g.place_block(block);
            });
            assert_eq!(g.is_grid_traversable(), t.1);
        });
    }

    #[test]
    fn can_player_reach_goal() {
        struct TestCase(Vec<Block>, Player, Player, bool);

        let tests = [
            TestCase(
                vec![],
                Player::new(String::from("top"), StartingPosition::Top),
                Player::new(String::from("bottom"), StartingPosition::Bottom),
                true,
            ),
            TestCase(
                vec![
                    Block::new(Position::new(0, 3), Orientation::Horizontal),
                    Block::new(Position::new(2, 3), Orientation::Horizontal),
                    Block::new(Position::new(4, 3), Orientation::Horizontal),
                    Block::new(Position::new(6, 3), Orientation::Horizontal),
                    Block::new(Position::new(7, 5), Orientation::Horizontal),
                ],
                Player::new(String::from("top"), StartingPosition::Top),
                Player::new(String::from("bottom"), StartingPosition::Bottom),
                true,
            ),
            TestCase(
                vec![
                    Block::new(Position::new(0, 3), Orientation::Horizontal),
                    Block::new(Position::new(2, 3), Orientation::Horizontal),
                    Block::new(Position::new(4, 3), Orientation::Horizontal),
                    Block::new(Position::new(6, 3), Orientation::Horizontal),
                    Block::new(Position::new(7, 5), Orientation::Horizontal),
                    Block::new(Position::new(6, 4), Orientation::Vertical),
                ],
                Player::new(String::from("top"), StartingPosition::Top),
                Player::new(String::from("bottom"), StartingPosition::Bottom),
                false,
            ),
        ];

        tests.iter().for_each(|t| {
            let mut g = Grid::new();
            t.0.iter().for_each(|block| {
                // assert!(g.place_block(block));
                // do this manually because place_block detects that this is illegal
                match block.orientation {
                    Orientation::Horizontal => {
                        g.horizontal_blocks[block.position.y]
                            [block.position.x] = Tile {
                            position: block.position,
                            state: TileState::Occupied,
                            tile_type: TileType::BlockPosition,
                        };
                    }
                    Orientation::Vertical => {
                        g.vertical_blocks[block.position.y][block.position.x] =
                            Tile {
                                position: block.position,
                                state: TileState::Occupied,
                                tile_type: TileType::BlockPosition,
                            };
                    }
                }
            });
            assert_eq!(t.3, g.can_player_reach_goal(&t.1));
            assert_eq!(t.3, g.can_player_reach_goal(&t.2));
        });
    }

    #[test]
    fn is_grid_traversable_with_pawn_on_origin() {
        // Regression for issue #3: BFS was seeded with a fabricated Free tile,
        // so an Occupied (0,0) was counted twice (82 != 81) -> false.
        let mut g = Grid::new();
        g.player_tiles[0][0].occupy();
        assert!(g.is_grid_traversable());

        // false branch survives: wall the occupied corner off
        g.horizontal_blocks[0][0].occupy();
        g.vertical_blocks[0][0].occupy();
        assert!(!g.is_grid_traversable());
    }

    // TODO (dio) illegal block tests
    #[test]
    fn can_not_place_illegal_blocks() {
        let mut g = Grid::new();
        let res = g.place_block(&Block::new(
            Position::new(0, 1),
            Orientation::Horizontal,
        ));
        assert!(res);

        //todo bug orientation
        let res = g.place_block(&Block::new(
            Position::new(1, 0),
            Orientation::Vertical,
        ));
        assert!(!res);
    }

    #[test]
    fn test_wall_crossing() {
        struct TestCase(Block, Block);

        let tests = [
            TestCase(
                Block {
                    orientation: Orientation::Horizontal,
                    is_placed: true,
                    position: Position { x: 0, y: 0 },
                },
                Block {
                    orientation: Orientation::Vertical,
                    is_placed: true,
                    position: Position { x: 0, y: 0 },
                },
            ),
            TestCase(
                Block {
                    orientation: Orientation::Horizontal,
                    is_placed: true,
                    position: Position { x: 1, y: 1 },
                },
                Block {
                    orientation: Orientation::Vertical,
                    is_placed: true,
                    position: Position { x: 1, y: 1 },
                },
            ),
            TestCase(
                Block {
                    orientation: Orientation::Horizontal,
                    is_placed: true,
                    position: Position { x: 3, y: 3 },
                },
                Block {
                    orientation: Orientation::Vertical,
                    is_placed: true,
                    position: Position { x: 3, y: 3 },
                },
            ),
        ];

        tests.iter().for_each(|t| {
            let mut grid = Grid::new();
            assert!(grid.place_block(&t.0));
            // TODO: fix this
            // assert_eq!(grid.place_block(t.1.position, t.1.orientation), false);
        })
    }
}
