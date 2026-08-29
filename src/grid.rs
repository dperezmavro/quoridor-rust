use crate::block::{Block, Orientation};
use crate::placement::Placement;
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
pub struct Grid<'a> {
    pub player_tiles: Box<[Box<[Tile; GRID_SIZE]>; GRID_SIZE]>,
    horizontal_blocks: Vec<&'a Block>,
    vertical_blocks: Vec<&'a Block>,
    horizontal_grid:
        Box<[[Tile; HORIZONTAL_BLOCK_MATRIX_X]; HORIZONTAL_BLOCK_MATRIX_Y]>,
    vertical_grid:
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

impl<'a> Grid<'a> {
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

        let mut horizontal_grid: Box<
            [[Tile; HORIZONTAL_BLOCK_MATRIX_X]; HORIZONTAL_BLOCK_MATRIX_Y],
        > = Box::new(
            [[Tile::default(); HORIZONTAL_BLOCK_MATRIX_X];
                HORIZONTAL_BLOCK_MATRIX_Y],
        );
        for lane in 0..HORIZONTAL_BLOCK_MATRIX_Y {
            for tile in 0..HORIZONTAL_BLOCK_MATRIX_X {
                horizontal_grid[lane][tile] = Tile::new(
                    TileType::BlockPosition,
                    TileState::Free,
                    Position { y: lane, x: tile },
                );
            }
        }

        let mut vertical_grid: Box<
            [[Tile; VERTICAL_BLOCK_MATRIX_X]; VERTICAL_BLOCK_MATRIX_Y],
        > = Box::new(
            [[Tile::default(); VERTICAL_BLOCK_MATRIX_X];
                VERTICAL_BLOCK_MATRIX_Y],
        );

        for lane in 0..VERTICAL_BLOCK_MATRIX_Y {
            for tile in 0..VERTICAL_BLOCK_MATRIX_X {
                vertical_grid[lane][tile] = Tile::new(
                    TileType::BlockPosition,
                    TileState::Free,
                    Position { y: lane, x: tile },
                );
            }
        }

        Self {
            player_tiles,
            horizontal_blocks: Vec::new(),
            vertical_blocks: Vec::new(),
            horizontal_grid,
            vertical_grid,
        }
    }

    pub fn get_reachable_neighbour_tiles(&self, p: &Position) -> Vec<&Tile> {
        assert!(p.x < GRID_SIZE && p.y < GRID_SIZE);

        let mut available_tiles: Vec<&Tile> = Vec::new();
        //top
        if p.y > 0 && self.horizontal_grid[p.y - 1][p.x].is_free() {
            available_tiles.push(&self.player_tiles[p.y - 1][p.x]);
        }
        //bottom
        if p.y < GRID_SIZE - 1 && self.horizontal_grid[p.y][p.x].is_free() {
            available_tiles.push(&self.player_tiles[p.y + 1][p.x]);
        }

        //left
        if p.x > 0 && self.vertical_grid[p.y][p.x - 1].is_free() {
            available_tiles.push(&self.player_tiles[p.y][p.x - 1]);
        }
        // right
        if p.x < GRID_SIZE - 1 && self.vertical_grid[p.y][p.x].is_free() {
            available_tiles.push(&self.player_tiles[p.y][p.x + 1]);
        }

        available_tiles
    }

    pub fn can_player_reach_goal(&self, p: &Player) -> bool {
        let goal_y = match p.starting_position {
            StartingPosition::Top => GRID_SIZE - 1,
            StartingPosition::Bottom => 0,
        };

        let mut visited: HashSet<&Position> = HashSet::new();
        let mut tiles_for_exploring: HashSet<&Position> = HashSet::new();

        tiles_for_exploring.insert(&p.position);

        while !tiles_for_exploring.is_empty() {
            // let neighbours = self.get_reachable_neighbour_tiles(p.position);
            let curr_pos = tiles_for_exploring.iter().cloned().next().unwrap();
            if visited.contains(curr_pos) {
                continue;
            }

            visited.insert(curr_pos);
            let neighbors = self.get_reachable_neighbour_tiles(curr_pos);

            for t in neighbors {
                if t.position.y == goal_y {
                    return true;
                }

                if !visited.contains(&t.position) {
                    tiles_for_exploring.insert(&t.position);
                }
            }

            tiles_for_exploring.remove(curr_pos);
        }

        false
    }

    fn free_block(&mut self, b: &Block) {
        let lane = b.placement.lane;
        let tile = b.placement.tile;
        match b.orientation {
            Orientation::Horizontal => {
                self.horizontal_grid[lane][tile].free();
                self.horizontal_grid[lane][tile + 1].free();
            }
            Orientation::Vertical => {
                self.vertical_grid[lane][tile].free();
                self.vertical_grid[lane + 1][tile].free();
            }
        };
    }

    fn does_intersect_existing_blocks(&self, incoming_block: &Block) -> bool {
        match incoming_block.orientation {
            Orientation::Horizontal => self
                .vertical_blocks
                .iter()
                .map(|x1| x1.placement.tile == incoming_block.placement.lane)
                .fold(false, |b1, b2| b1 || b2),
            Orientation::Vertical => {
                println!(
                    "existing blocks \n    {}\n    {:?}\n    incoming: {:?}",
                    self.horizontal_blocks.len(),
                    self.horizontal_blocks.clone(),
                    incoming_block
                );
                let res = self.horizontal_blocks.iter().map(|b| {
                    incoming_block.placement.lane == b.placement.tile
                        && incoming_block.placement.tile + 1 > b.placement.lane
                });

                println!("res: {:?}", res.clone().collect::<Vec<bool>>());

                let res2 = res.reduce(|b1, b2| b1 || b2).unwrap_or(false);
                println!("folding: {}", res2,);

                res2
            }
        }
    }

    fn add_block(&mut self, b: &Block) {
        let tile = b.placement.tile;
        let lane = b.placement.lane;
        match b.orientation {
            Orientation::Horizontal => {
                self.horizontal_grid[lane][tile].occupy();
                self.horizontal_grid[lane][tile + 1].occupy();
            }
            Orientation::Vertical => {
                self.vertical_grid[lane][tile].occupy();
                self.vertical_grid[lane + 1][tile].occupy();
            }
        }
    }

    fn is_block_position_free(&self, b: &Block) -> bool {
        let target_list = match b.orientation {
            Orientation::Horizontal => &self.horizontal_blocks,
            Orientation::Vertical => &self.vertical_blocks,
        };
        target_list
            .iter()
            .filter(|t| {
                t.placement.lane == b.placement.lane
                    && b.placement.tile.abs_diff(t.placement.tile) <= 1
            })
            .count()
            == 0
    }

    pub fn place_block(
        &mut self,
        p1: &Player,
        p2: &Player,
        b: &'a Block,
    ) -> bool {
        let p = b.placement;
        assert!(p.lane < GRID_SIZE - 1 && p.tile < GRID_SIZE - 1);

        if !self.does_intersect_existing_blocks(b)
            && self.is_block_position_free(b)
        {
            self.add_block(b);

            if !(self.can_player_reach_goal(&p1)
                && self.can_player_reach_goal(&p2))
            {
                self.free_block(b);
                return false;
            }

            // add the block to the individual block tracking lists
            match b.orientation {
                Orientation::Horizontal => self.horizontal_blocks.push(b),
                Orientation::Vertical => self.vertical_blocks.push(b),
            }
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /********************************************/
    //         place_block
    /********************************************/
    #[test]
    #[should_panic]
    fn test_place_block_out_of_bounds() {
        let tests = [
            Block::new(Placement::new(8, 0), Orientation::Horizontal),
            Block::new(Placement::new(0, 8), Orientation::Horizontal),
        ];
        for b in tests.iter() {
            let mut grid = Grid::new();
            let p1 = Player::new(String::new(), StartingPosition::Top);
            let p2 = Player::new(String::new(), StartingPosition::Bottom);
            let success = grid.place_block(&p1, &p2, &b);
            assert!(!success);
        }
    }

    #[test]
    fn place_block_explicit() {
        struct TestCase(Block, bool);

        let tests = [
            TestCase(
                Block::new(Placement::new(0, 0), Orientation::Horizontal),
                true,
            ),
            TestCase(
                Block::new(Placement::new(1, 0), Orientation::Horizontal),
                false,
            ),
            TestCase(
                Block::new(Placement::new(3, 0), Orientation::Horizontal),
                true,
            ),
            TestCase(
                Block::new(Placement::new(3, 0), Orientation::Horizontal),
                false,
            ),
        ];

        let mut grid = Grid::new();
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);
        for t in tests.iter() {
            let result = grid.place_block(&p1, &p2, &t.0);
            assert_eq!(t.1, result);
        }
    }

    #[test]
    fn test_bulk_place_blocks_illegal() {
        let tests = [vec![
            Block::new(Placement::new(0, 3), Orientation::Horizontal),
            Block::new(Placement::new(2, 3), Orientation::Horizontal),
            Block::new(Placement::new(4, 3), Orientation::Horizontal),
            Block::new(Placement::new(6, 3), Orientation::Horizontal),
            Block::new(Placement::new(7, 5), Orientation::Horizontal),
            Block::new(Placement::new(6, 4), Orientation::Vertical),
        ]];

        for t in tests.iter() {
            let mut g = Grid::new();
            let p1 = Player::new(String::new(), StartingPosition::Top);
            let p2 = Player::new(String::new(), StartingPosition::Bottom);
            let results: bool = t
                .iter()
                .map(|b| g.place_block(&p1, &p2, b))
                .fold(true, |b1, x| b1 && x);
            assert!(!results);
        }
    }
    #[test]
    fn test_bulk_place_blocks_legal() {
        let tests = [vec![
            Block::new(Placement::new(0, 3), Orientation::Horizontal),
            Block::new(Placement::new(2, 3), Orientation::Horizontal),
            Block::new(Placement::new(4, 3), Orientation::Horizontal),
            Block::new(Placement::new(6, 3), Orientation::Horizontal),
            Block::new(Placement::new(7, 5), Orientation::Horizontal),
        ]];

        for t in tests.iter() {
            let mut g = Grid::new();
            let p1 = Player::new(String::new(), StartingPosition::Top);
            let p2 = Player::new(String::new(), StartingPosition::Bottom);

            let results: bool = t
                .iter()
                .map(|b| g.place_block(&p1, &p2, b))
                .fold(true, |b1, x| b1 && x);
            assert!(results);
        }
    }

    /********************************************/
    //         can_player_reach_goal
    /********************************************/
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
            let result = grid
                .get_reachable_neighbour_tiles(&Position { x: t.0, y: t.1 });
            assert_eq!(t.2, result.len());
        });
    }

    #[test]
    fn get_reachable_neighbour_tiles_with_blocks() {
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);

        // place horizontal block
        let mut g = Grid::new();
        let result = g.get_reachable_neighbour_tiles(&Position::new(0, 0));
        assert_eq!(result.len(), 2);
        let b = Block::new(Placement::new(0, 0), Orientation::Horizontal);
        let placed = g.place_block(&p1, &p2, &b);
        assert!(placed);
        let result = g.get_reachable_neighbour_tiles(&Position::new(0, 0));
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].position, Position::new(1, 0));

        // place vertical block
        let mut g = Grid::new();
        let b = Block::new(Placement::new(0, 0), Orientation::Vertical);
        let placed = g.place_block(&p1, &p2, &b);
        assert!(placed);
        let result = g.get_reachable_neighbour_tiles(&Position::new(0, 0));
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].position, Position::new(0, 1));
    }

    #[test]
    fn get_reachable_neighbour_tiles_assert() {
        let g = Grid::new();
        let result = g.get_reachable_neighbour_tiles(&Position::new(0, 0));
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].position.x, 0);
        assert_eq!(result[0].position.y, 1);

        assert_eq!(result[1].position.x, 1);
        assert_eq!(result[1].position.y, 0);
    }

    /********************************************/
    //         can_player_reach_goal
    /********************************************/
    #[test]
    fn can_players_reach_end() {
        let mut g = Grid::new();
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);
        let block = &Block::default();
        assert!(g.place_block(&p1, &p2, block));
        assert!(g.can_player_reach_goal(&p1));
        assert!(g.can_player_reach_goal(&p2));
    }

    #[test]
    fn test_can_player_reach_goal_normal() {
        struct TestCase(Vec<Block>, Player, Player);

        let tests = [
            TestCase(
                vec![],
                Player::new(String::from("top"), StartingPosition::Top),
                Player::new(String::from("bottom"), StartingPosition::Bottom),
            ),
            TestCase(
                vec![
                    Block::new(Placement::new(0, 3), Orientation::Horizontal),
                    Block::new(Placement::new(2, 3), Orientation::Horizontal),
                    Block::new(Placement::new(4, 3), Orientation::Horizontal),
                    Block::new(Placement::new(6, 3), Orientation::Horizontal),
                    Block::new(Placement::new(7, 5), Orientation::Horizontal),
                ],
                Player::new(String::from("top"), StartingPosition::Top),
                Player::new(String::from("bottom"), StartingPosition::Bottom),
            ),
            TestCase(
                vec![
                    Block::new(Placement::new(0, 3), Orientation::Horizontal),
                    Block::new(Placement::new(2, 3), Orientation::Horizontal),
                    Block::new(Placement::new(4, 3), Orientation::Horizontal),
                    Block::new(Placement::new(6, 3), Orientation::Horizontal),
                    Block::new(Placement::new(7, 5), Orientation::Horizontal),
                    Block::new(Placement::new(6, 4), Orientation::Vertical),
                ],
                Player {
                    position: Position::new(4, 4),
                    name: String::new(),
                    id: String::new(),
                    blocks: [Block::default(); 10],
                    starting_position: StartingPosition::Top,
                },
                Player {
                    position: Position::new(4, 3),
                    name: String::new(),
                    id: String::new(),
                    blocks: [Block::default(); 10],
                    starting_position: StartingPosition::Bottom,
                },
            ),
        ];

        tests.iter().for_each(|t| {
            let mut g = Grid::new();
            t.0.iter().for_each(|block| {
                assert!(g.place_block(&t.1, &t.2, block));
            });

            g.player_tiles[t.1.position.y][t.1.position.x].occupy();
            g.player_tiles[t.2.position.y][t.2.position.x].occupy();

            assert!(g.can_player_reach_goal(&t.1));
            assert!(g.can_player_reach_goal(&t.2));
        });
    }

    #[test]
    fn test_can_player_reach_goal_disallowed_block_formations() {
        struct TestCase(Vec<Block>, Player, Player);

        let tests = [
            TestCase(
                vec![
                    Block::new(Placement::new(0, 3), Orientation::Horizontal),
                    Block::new(Placement::new(2, 3), Orientation::Horizontal),
                    Block::new(Placement::new(4, 3), Orientation::Horizontal),
                    Block::new(Placement::new(6, 3), Orientation::Horizontal),
                    Block::new(Placement::new(7, 5), Orientation::Horizontal),
                    Block::new(Placement::new(6, 4), Orientation::Vertical),
                ],
                Player::new(String::from("top"), StartingPosition::Top),
                Player::new(String::from("bottom"), StartingPosition::Bottom),
            ),
            TestCase(
                vec![
                    Block::new(Placement::new(0, 3), Orientation::Horizontal),
                    Block::new(Placement::new(2, 3), Orientation::Horizontal),
                    Block::new(Placement::new(4, 3), Orientation::Horizontal),
                    Block::new(Placement::new(6, 3), Orientation::Horizontal),
                    Block::new(Placement::new(7, 5), Orientation::Horizontal),
                    Block::new(Placement::new(6, 4), Orientation::Vertical),
                ],
                Player {
                    position: Position::new(4, 3),
                    name: String::new(),
                    id: String::new(),
                    blocks: [Block::default(); 10],
                    starting_position: StartingPosition::Top,
                },
                Player {
                    position: Position::new(4, 6),
                    name: String::new(),
                    id: String::new(),
                    blocks: [Block::default(); 10],
                    starting_position: StartingPosition::Bottom,
                },
            ),
        ];

        tests.iter().for_each(|t| {
            let mut g = Grid::new();
            t.0.iter().for_each(|block| {
                // assert!(g.place_block(block));
                // do this manually because place_block detects that this is illegal
                match block.orientation {
                    Orientation::Horizontal => {
                        g.horizontal_grid[block.placement.lane]
                            [block.placement.tile] = Tile {
                            position: Position::new(
                                block.placement.tile,
                                block.placement.lane,
                            ),
                            state: TileState::Occupied,
                            tile_type: TileType::BlockPosition,
                        };
                        g.horizontal_grid[block.placement.lane]
                            [block.placement.tile + 1] = Tile {
                            position: Position::new(
                                block.placement.tile,
                                block.placement.lane,
                            ),
                            state: TileState::Occupied,
                            tile_type: TileType::BlockPosition,
                        };
                    }
                    Orientation::Vertical => {
                        g.vertical_grid[block.placement.lane]
                            [block.placement.tile] = Tile {
                            position: Position::new(
                                block.placement.tile,
                                block.placement.lane,
                            ),
                            state: TileState::Occupied,
                            tile_type: TileType::BlockPosition,
                        };
                        g.vertical_grid[block.placement.lane + 1]
                            [block.placement.tile] = Tile {
                            position: Position::new(
                                block.placement.tile,
                                block.placement.lane,
                            ),
                            state: TileState::Occupied,
                            tile_type: TileType::BlockPosition,
                        };
                    }
                }
            });
            g.player_tiles[t.1.position.y][t.1.position.x].occupy();
            g.player_tiles[t.2.position.y][t.2.position.x].occupy();

            assert!(!g.can_player_reach_goal(&t.1));
            assert!(!g.can_player_reach_goal(&t.2));
        });
    }

    /********************************************/
    //         place_block
    /********************************************/
    #[test]
    fn can_place_weird_blocks() {
        let b1 = Block::new(Placement::new(0, 1), Orientation::Horizontal);
        let b2 = Block::new(Placement::new(0, 1), Orientation::Vertical);
        let b3 = Block::new(Placement::new(0, 0), Orientation::Vertical);
        let mut g = Grid::new();
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);

        assert!(!g.does_intersect_existing_blocks(&b1));
        assert!(g.place_block(&p1, &p2, &b1));
        assert!(!g.does_intersect_existing_blocks(&b2));
        assert!(g.place_block(&p1, &p2, &b2));
        assert!(!g.does_intersect_existing_blocks(&b3));
        assert!(g.place_block(&p1, &p2, &b3));
    }

    /********************************************/
    //         does_intersect_existing_blocks
    /********************************************/
    #[test]
    fn verify_does_intersect_existing_blocks() {
        // setup
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);

        let mut g = Grid::new();
        let b1 = Block::new(Placement::new(4, 3), Orientation::Horizontal);
        let b2 = Block::new(Placement::new(3, 4), Orientation::Vertical);
        assert!(g.place_block(&p1, &p2, &b1));
        assert!(g.does_intersect_existing_blocks(&b2));
        assert!(!g.place_block(&p1, &p2, &b2));

        let mut g = Grid::new();
        let b1 = Block::new(Placement::new(3, 4), Orientation::Vertical);
        let b2 = Block::new(Placement::new(4, 3), Orientation::Horizontal);
        assert!(g.place_block(&p1, &p2, &b1));
        assert!(g.does_intersect_existing_blocks(&b2));
        assert!(!g.place_block(&p1, &p2, &b2));

        let mut g = Grid::new();
        let b1 = Block::new(Placement::new(0, 0), Orientation::Horizontal);
        let b2 = Block::new(Placement::new(0, 0), Orientation::Vertical);
        assert!(g.place_block(&p1, &p2, &b1));
        assert!(g.does_intersect_existing_blocks(&b2));
        assert!(!g.place_block(&p1, &p2, &b2));
    }

    #[test]
    fn verify_blocks_do_not_intersect() {
        let mut g = Grid::new();
        let p1 = &Player::new(String::new(), StartingPosition::Top);
        let p2 = &Player::new(String::new(), StartingPosition::Bottom);
        let b1 = &Block {
            orientation: Orientation::Vertical,
            is_placed: true,
            placement: Placement::new(7, 7),
        };
        let b2 = Block {
            orientation: Orientation::Horizontal,
            is_placed: true,
            placement: Placement::new(5, 7),
        };
        let b3 = &Block {
            orientation: Orientation::Horizontal,
            is_placed: true,
            placement: Placement::new(5, 7),
        };
        let b4 = &Block {
            orientation: Orientation::Horizontal,
            is_placed: true,
            placement: Placement::new(5, 6),
        };
        assert!(g.place_block(p1, p2, b1));
        assert!(!g.does_intersect_existing_blocks(&b2));
        assert!(!g.does_intersect_existing_blocks(&b3));
        assert!(!g.does_intersect_existing_blocks(&b4));
    }

    #[test]
    fn verify_cant_place_crossing_blocks() {
        struct TestCase(Block, Block);

        let tests = [
            TestCase(
                Block {
                    orientation: Orientation::Horizontal,
                    is_placed: true,
                    placement: Placement::new(0, 0),
                },
                Block {
                    orientation: Orientation::Vertical,
                    is_placed: true,
                    placement: Placement::new(0, 0),
                },
            ),
            TestCase(
                Block {
                    orientation: Orientation::Horizontal,
                    is_placed: true,
                    placement: Placement::new(1, 1),
                },
                Block {
                    orientation: Orientation::Vertical,
                    is_placed: true,
                    placement: Placement::new(1, 1),
                },
            ),
            TestCase(
                Block {
                    orientation: Orientation::Horizontal,
                    is_placed: true,
                    placement: Placement::new(3, 3),
                },
                Block {
                    orientation: Orientation::Vertical,
                    is_placed: true,
                    placement: Placement::new(3, 3),
                },
            ),
        ];

        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);
        for t in tests.iter() {
            let mut grid = Grid::new();
            assert!(grid.place_block(&p1, &p2, &t.0));
            assert!(!grid.place_block(&p1, &p2, &t.1));
        }
    }

    #[test]
    fn verify_can_place_t_junctions() {
        struct TestCase(Block, Block);

        let tests = [
            TestCase(
                Block {
                    orientation: Orientation::Horizontal,
                    is_placed: true,
                    placement: Placement::new(0, 0),
                },
                Block {
                    orientation: Orientation::Vertical,
                    is_placed: true,
                    placement: Placement::new(0, 1),
                },
            ),
            TestCase(
                Block {
                    orientation: Orientation::Vertical,
                    is_placed: true,
                    placement: Placement::new(0, 0),
                },
                Block {
                    orientation: Orientation::Horizontal,
                    is_placed: true,
                    placement: Placement::new(1, 0),
                },
            ),
            TestCase(
                Block {
                    orientation: Orientation::Vertical,
                    is_placed: true,
                    placement: Placement::new(7, 7),
                },
                Block {
                    orientation: Orientation::Horizontal,
                    is_placed: true,
                    placement: Placement::new(5, 6),
                },
            ),
            TestCase(
                Block {
                    orientation: Orientation::Vertical,
                    is_placed: true,
                    placement: Placement::new(7, 7),
                },
                Block {
                    orientation: Orientation::Horizontal,
                    is_placed: true,
                    placement: Placement::new(5, 7),
                },
            ),
            TestCase(
                Block {
                    orientation: Orientation::Vertical,
                    is_placed: true,
                    placement: Placement::new(7, 7),
                },
                Block {
                    orientation: Orientation::Horizontal,
                    is_placed: true,
                    placement: Placement::new(7, 6),
                },
            ),
        ];

        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);
        for t in tests {
            let mut grid = Grid::new();
            assert!(grid.place_block(&p1, &p2, &t.0));
            assert!(grid.place_block(&p1, &p2, &t.1));
        }
    }
}
