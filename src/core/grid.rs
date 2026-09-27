use crate::core::block::Block;
use crate::core::orientation::Orientation;
use crate::core::player::{NUM_BLOCKS, Player};
use crate::core::position::Position;
use crate::core::tile::{Tile, TileState, TileType};
use std::collections::HashSet;

pub const NUM_TILES: usize = 9;

#[derive(Debug)]
pub struct Grid {
    pub player_tiles: Box<[Box<[Tile; NUM_TILES]>; NUM_TILES]>,
    horizontal_blocks: Vec<Block>,
    vertical_blocks: Vec<Block>,
}

fn generate_row(y: usize) -> Box<[Tile; NUM_TILES]> {
    let mut res = Box::new([Tile::default(); NUM_TILES]);
    let default = Tile::default();
    for i in 0..NUM_TILES {
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
                x: NUM_TILES / 2,
            },
        );
        let player_2_tile = Tile::new(
            TileType::Playable,
            TileState::Occupied,
            Position {
                y: NUM_TILES - 1,
                x: NUM_TILES / 2,
            },
        );
        let mut first_row = generate_row(0);
        first_row[NUM_TILES / 2] = player_1_tile;
        let mut last_row = generate_row(NUM_TILES - 1);
        last_row[NUM_TILES / 2] = player_2_tile;

        let player_tiles: Box<[Box<[Tile; NUM_TILES]>; NUM_TILES]> =
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

        Self {
            player_tiles,
            horizontal_blocks: Vec::with_capacity(NUM_BLOCKS),
            vertical_blocks: Vec::with_capacity(NUM_BLOCKS),
        }
    }

    pub fn get_reachable_neighbour_tiles(&self, p: &Position) -> Vec<&Tile> {
        let mut available_tiles: Vec<&Tile> = Vec::new();
        //top
        if p.y > 0
            && self.horizontal_blocks.iter().all(|x| {
                !(x.placement.lane == p.y - 1
                    && (x.placement.tile == p.x || x.placement.tile + 1 == p.x))
            })
        {
            available_tiles.push(&self.player_tiles[p.y - 1][p.x]);
        }
        //bottom
        if p.y < NUM_TILES - 1
            && self.horizontal_blocks.iter().all(|x| {
                !(x.placement.lane == p.y
                    && (x.placement.tile == p.x || x.placement.tile + 1 == p.x))
            })
        {
            available_tiles.push(&self.player_tiles[p.y + 1][p.x]);
        }

        //left
        if p.x > 0
            && self.vertical_blocks.iter().all(|x| {
                !(x.placement.lane == p.x - 1
                    && (x.placement.tile == p.y || x.placement.tile + 1 == p.y))
            })
        {
            available_tiles.push(&self.player_tiles[p.y][p.x - 1]);
        }
        // right
        if p.x < NUM_TILES - 1
            && self.vertical_blocks.iter().all(|x| {
                !(x.placement.lane == p.x
                    && (x.placement.tile == p.y || x.placement.tile + 1 == p.y))
            })
        {
            available_tiles.push(&self.player_tiles[p.y][p.x + 1]);
        }

        available_tiles
    }

    pub fn can_player_reach_goal(&self, p: &Player) -> bool {
        let mut visited: HashSet<&Position> = HashSet::new();
        let mut tiles_for_exploring: HashSet<&Position> = HashSet::new();

        tiles_for_exploring.insert(&p.position);

        while !tiles_for_exploring.is_empty() {
            let curr_pos = tiles_for_exploring.iter().cloned().next().unwrap();
            if visited.contains(curr_pos) {
                continue;
            }

            visited.insert(curr_pos);
            let neighbors = self.get_reachable_neighbour_tiles(curr_pos);

            for t in neighbors {
                if t.position.y == p.get_y_goal() {
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
        match b.orientation {
            Orientation::Horizontal => {
                self.horizontal_blocks.pop().unwrap();
            }
            Orientation::Vertical => {
                self.vertical_blocks.pop().unwrap();
            }
        };
    }

    fn does_intersect_existing_blocks(&self, incoming_block: &Block) -> bool {
        // applied to an incoming horizontal block
        // testing for an existing vertical block
        let hor_clos = |x1: &&Block| -> bool {
            x1.placement.tile == incoming_block.placement.lane
                && x1.placement.lane == incoming_block.placement.tile
        };

        // applied to an incoming vertical block
        // testing for an existing horizontal block
        let ver_clos = |x1: &&Block| -> bool {
            x1.placement.tile == incoming_block.placement.lane
                && x1.placement.lane.abs_diff(incoming_block.placement.tile)
                    == 0
        };
        match incoming_block.orientation {
            Orientation::Horizontal => {
                self.vertical_blocks.iter().filter(hor_clos).count() != 0
            }
            Orientation::Vertical => {
                self.horizontal_blocks.iter().filter(ver_clos).count() != 0
            }
        }
    }

    fn add_block(&mut self, b: Block) {
        match b.orientation {
            Orientation::Horizontal => {
                self.horizontal_blocks.push(b);
            }
            Orientation::Vertical => {
                self.vertical_blocks.push(b);
            }
        }
    }

    fn is_block_position_free(&self, b: &Block) -> bool {
        let target_list = match b.orientation {
            Orientation::Horizontal => &self.horizontal_blocks,
            Orientation::Vertical => &self.vertical_blocks,
        };
        target_list.iter().all(|t| {
            !(t.placement.lane == b.placement.lane
                && b.placement.tile.abs_diff(t.placement.tile) <= 1)
        })
    }

    pub fn place_block(&mut self, p1: &Player, p2: &Player, b: Block) -> bool {
        if self.can_place_block(p1, p2, b) {
            self.add_block(b);
            return true;
        }
        false
    }

    pub fn can_place_block(
        &mut self,
        p1: &Player,
        p2: &Player,
        b: Block,
    ) -> bool {
        if b.placement.tile >= NUM_TILES - 1
            || b.placement.lane >= NUM_TILES - 1
        {
            return false;
        }
        if !self.does_intersect_existing_blocks(&b)
            && self.is_block_position_free(&b)
        {
            self.add_block(b);

            let res = self.can_player_reach_goal(p1)
                && self.can_player_reach_goal(p2);
            self.free_block(&b);
            return res;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::placement::Placement;
    use crate::core::player::StartingPosition;

    /********************************************/
    //         place_block
    /********************************************/
    #[test]
    fn test_place_block_out_of_bounds() {
        let tests = [
            Block::new(Placement::new(8, 0), Orientation::Horizontal),
            Block::new(Placement::new(0, 8), Orientation::Horizontal),
        ];
        for b in tests.iter() {
            let mut grid = Grid::new();
            let p1 = Player::new(String::new(), StartingPosition::Top);
            let p2 = Player::new(String::new(), StartingPosition::Bottom);
            let success = grid.place_block(&p1, &p2, *b);
            assert!(!success);
        }
    }

    #[test]
    fn place_block_explicit() {
        let mut grid = Grid::new();
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);

        let b1 = Block::new(Placement::new(0, 0), Orientation::Horizontal);
        assert!(grid.place_block(&p1, &p2, b1));

        let b2 = Block::new(Placement::new(1, 0), Orientation::Horizontal);
        assert!(!grid.place_block(&p1, &p2, b2));

        let b3 = Block::new(Placement::new(3, 0), Orientation::Horizontal);
        assert!(grid.place_block(&p1, &p2, b3));
        assert!(!grid.place_block(&p1, &p2, b3));
    }

    #[test]
    fn test_bulk_place_blocks_illegal() {
        let tests = vec![
            Block::new(Placement::new(0, 3), Orientation::Horizontal),
            Block::new(Placement::new(2, 3), Orientation::Horizontal),
            Block::new(Placement::new(4, 3), Orientation::Horizontal),
            Block::new(Placement::new(6, 3), Orientation::Horizontal),
            Block::new(Placement::new(7, 5), Orientation::Horizontal),
        ];

        let mut g = Grid::new();
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);
        let results: bool = tests.iter().all(|b| g.place_block(&p1, &p2, *b));
        assert!(results);

        // block that makes the grid illegal
        let b = Block::new(Placement::new(4, 6), Orientation::Vertical);
        assert!(!g.can_place_block(&p1, &p2, b));
        assert!(!g.place_block(&p1, &p2, b));
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

            let results: bool = t.iter().all(|b| g.place_block(&p1, &p2, *b));
            assert!(results);
        }
    }

    /********************************************/
    //         get_reachable_neighbour_tiles
    /********************************************/
    #[test]
    fn get_reachable_neighbour_tiles() {
        struct TestCase(usize, usize, usize);
        let tests = vec![
            TestCase(0, 0, 2),
            TestCase(1, 1, 4),
            TestCase(NUM_TILES - 1, NUM_TILES - 1, 2),
            TestCase(0, NUM_TILES - 1, 2),
            TestCase(NUM_TILES - 1, 0, 2),
            TestCase(NUM_TILES - 3, NUM_TILES - 3, 4),
            TestCase(NUM_TILES - 2, 0, 3),
            TestCase(0, NUM_TILES - 2, 3),
            TestCase(NUM_TILES - 1, NUM_TILES - 2, 3),
            TestCase(NUM_TILES - 2, NUM_TILES - 1, 3),
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
        assert!(g.place_block(&p1, &p2, b));
        let result = g.get_reachable_neighbour_tiles(&Position::new(0, 0));
        assert_eq!(1, result.len());
        assert_eq!(result[0].position, Position::new(1, 0));

        // place vertical block
        let mut g = Grid::new();
        let b = Block::new(Placement::new(0, 0), Orientation::Vertical);
        assert!(g.place_block(&p1, &p2, b));
        let result = g.get_reachable_neighbour_tiles(&Position::new(0, 0));
        assert_eq!(1, result.len());
        assert_eq!(Position::new(0, 1), result[0].position);

        // place vertical block mid-grid
        let mut g = Grid::new();
        let b = Block::new(Placement::new(4, 0), Orientation::Horizontal);
        assert!(g.place_block(&p1, &p2, b));
        let result = g.get_reachable_neighbour_tiles(&Position::new(4, 0));
        assert_eq!(2, result.len());
        // // only bottom and right
        assert_eq!(Position::new(3, 0), result[0].position);
        assert_eq!(Position::new(5, 0), result[1].position);

        // place vertical block mid-grid
        let mut g = Grid::new();
        let b = Block::new(Placement::new(0, 3), Orientation::Vertical);
        assert!(g.place_block(&p1, &p2, b));
        let result = g.get_reachable_neighbour_tiles(&Position::new(4, 0));
        assert_eq!(2, result.len());
        // only bottom and right
        assert_eq!(Position::new(4, 1), result[0].position);
        assert_eq!(Position::new(5, 0), result[1].position);
    }

    #[test]
    fn get_reachable_neighbour_tiles_with_blocks_middle_of_the_board() {
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);
        let mut p3 = Player::new(String::new(), StartingPosition::Bottom);
        p3.position = Position { x: 4, y: 1 };
        let b1 = Block::new(Placement::new(0, 3), Orientation::Vertical);
        let b2 = Block::new(Placement::new(0, 4), Orientation::Vertical);

        // place vertical block mid-grid
        let mut g = Grid::new();
        assert!(g.place_block(&p1, &p2, b1));
        assert!(g.place_block(&p1, &p2, b2));
        let result = g.get_reachable_neighbour_tiles(&p1.position);
        assert_eq!(1, result.len());
        // only bottom and right
        assert_eq!(Position::new(4, 1), result[0].position);

        let mut g = Grid::new();
        assert!(g.place_block(&p1, &p2, b1));
        assert!(g.place_block(&p1, &p2, b2));
        let result = g.get_reachable_neighbour_tiles(&p3.position);
        assert_eq!(2, result.len());
        assert_eq!(
            vec![Position::new(4, 0), Position::new(4, 2)],
            result.iter().map(|t| t.position).collect::<Vec<Position>>()
        );
    }

    #[test]
    fn get_reachable_neighbour_tiles_assert() {
        let g = Grid::new();
        let result = g.get_reachable_neighbour_tiles(&Position::new(0, 0));
        assert_eq!(2, result.len());
        assert_eq!(0, result[0].position.x);
        assert_eq!(1, result[0].position.y);

        assert_eq!(1, result[1].position.x);
        assert_eq!(0, result[1].position.y);
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
        assert!(g.place_block(&p1, &p2, *block));
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
                    Block::new(Placement::new(4, 6), Orientation::Vertical),
                ],
                Player {
                    position: Position::new(4, 4),
                    name: String::new(),
                    starting_position: StartingPosition::Top,
                    remaining_blocks: NUM_BLOCKS,
                },
                Player {
                    position: Position::new(4, 3),
                    name: String::new(),
                    starting_position: StartingPosition::Bottom,
                    remaining_blocks: NUM_BLOCKS,
                },
            ),
        ];

        tests.iter().for_each(|t| {
            let mut g = Grid::new();
            g.player_tiles[t.1.position.y][t.1.position.x].occupy();
            g.player_tiles[t.2.position.y][t.2.position.x].occupy();

            t.0.iter().for_each(|block| {
                assert!(g.can_place_block(&t.1, &t.2, *block), "{:?}", block);
                assert!(g.place_block(&t.1, &t.2, *block), "{:?}", block);
            });

            assert!(g.can_player_reach_goal(&t.1), "player 1 {:?}", t.1);
            assert!(g.can_player_reach_goal(&t.2), "player 2 {:?}", t.2);
        });
    }

    #[test]
    fn test_can_player_reach_goal_disallowed_block_formations_from_correct_position()
     {
        struct TestCase(Player, Player);
        let blocks = vec![
            Block::new(Placement::new(0, 3), Orientation::Horizontal),
            Block::new(Placement::new(2, 3), Orientation::Horizontal),
            Block::new(Placement::new(4, 3), Orientation::Horizontal),
            Block::new(Placement::new(6, 3), Orientation::Horizontal),
            Block::new(Placement::new(7, 5), Orientation::Horizontal),
            Block::new(Placement::new(4, 6), Orientation::Vertical),
        ];
        let mut g = Grid::new();
        blocks.iter().for_each(|block| match block.orientation {
            Orientation::Horizontal => {
                g.horizontal_blocks.push(block.clone());
            }
            Orientation::Vertical => {
                g.vertical_blocks.push(block.clone());
            }
        });

        // test players can reach from end
        let mut p1 = Player::new(String::from("t"), StartingPosition::Top);
        p1.position = Position::new(4, 4);
        let mut p2 = Player::new(String::from("b"), StartingPosition::Bottom);
        p2.position = Position::new(4, 3);

        assert!(g.can_player_reach_goal(&p1));
        assert!(g.can_player_reach_goal(&p2));
    }

    #[test]
    fn test_can_place_block_after_players_move() {
        struct TestCase(Player, Player);
        let blocks = vec![
            Block::new(Placement::new(0, 3), Orientation::Horizontal),
            Block::new(Placement::new(2, 3), Orientation::Horizontal),
            Block::new(Placement::new(4, 3), Orientation::Horizontal),
            Block::new(Placement::new(6, 3), Orientation::Horizontal),
            Block::new(Placement::new(7, 5), Orientation::Horizontal),
        ];
        let mut g = Grid::new();
        let mut p1 = Player::new(String::from("t"), StartingPosition::Top);
        let mut p2 = Player::new(String::from("b"), StartingPosition::Bottom);

        assert!(blocks.iter().all(|block| g.place_block(
            &p1,
            &p2,
            block.clone()
        )));

        let block_dividing_board =
            Block::new(Placement::new(4, 6), Orientation::Vertical);
        assert!(!g.place_block(&p1, &p2, block_dividing_board));
        assert!(g.can_player_reach_goal(&p1));
        assert!(g.can_player_reach_goal(&p2));

        p1.position = Position::new(4, 4);
        p2.position = Position::new(4, 3);
        assert!(g.place_block(&p1, &p2, block_dividing_board));
        assert!(g.can_player_reach_goal(&p1));
        assert!(g.can_player_reach_goal(&p2));
    }

    #[test]
    fn test_can_not_player_reach_goal_disallowed_block_formations() {
        struct TestCase(Player, Player);
        let blocks = vec![
            Block::new(Placement::new(0, 3), Orientation::Horizontal),
            Block::new(Placement::new(2, 3), Orientation::Horizontal),
            Block::new(Placement::new(4, 3), Orientation::Horizontal),
            Block::new(Placement::new(6, 3), Orientation::Horizontal),
            Block::new(Placement::new(7, 5), Orientation::Horizontal),
            Block::new(Placement::new(4, 6), Orientation::Vertical),
        ];
        let mut g = Grid::new();
        blocks.iter().for_each(|block| match block.orientation {
            Orientation::Horizontal => {
                g.horizontal_blocks.push(block.clone());
            }
            Orientation::Vertical => {
                g.vertical_blocks.push(block.clone());
            }
        });

        // test players can't reach from start
        let p1 = Player::new(String::from("t"), StartingPosition::Top);
        let p2 = Player::new(String::from("b"), StartingPosition::Bottom);

        let neighbours = g.get_reachable_neighbour_tiles(&p1.position);
        assert!(!g.can_player_reach_goal(&p1));
        assert!(!g.can_player_reach_goal(&p2));
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
        assert!(g.place_block(&p1, &p2, b1));
        assert!(!g.does_intersect_existing_blocks(&b2));
        assert!(g.place_block(&p1, &p2, b2));
        assert!(!g.does_intersect_existing_blocks(&b3));
        assert!(g.place_block(&p1, &p2, b3));
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
        assert!(g.place_block(&p1, &p2, b1));
        assert!(g.does_intersect_existing_blocks(&b2));
        assert!(!g.place_block(&p1, &p2, b2));

        let mut g = Grid::new();
        let b1 = Block::new(Placement::new(3, 4), Orientation::Vertical);
        let b2 = Block::new(Placement::new(4, 3), Orientation::Horizontal);
        assert!(g.place_block(&p1, &p2, b1));
        assert!(g.does_intersect_existing_blocks(&b2));
        assert!(!g.place_block(&p1, &p2, b2));

        let mut g = Grid::new();
        let b1 = Block::new(Placement::new(0, 0), Orientation::Horizontal);
        let b2 = Block::new(Placement::new(0, 0), Orientation::Vertical);
        assert!(g.place_block(&p1, &p2, b1));
        assert!(g.does_intersect_existing_blocks(&b2));
        assert!(!g.place_block(&p1, &p2, b2));
    }

    #[test]
    fn verify_blocks_do_not_intersect() {
        let mut g = Grid::new();
        let p1 = &Player::new(String::new(), StartingPosition::Top);
        let p2 = &Player::new(String::new(), StartingPosition::Bottom);
        let b1 = &Block {
            orientation: Orientation::Vertical,

            placement: Placement::new(7, 7),
        };
        let b2 = &Block {
            orientation: Orientation::Horizontal,

            placement: Placement::new(5, 7),
        };
        let b3 = &Block {
            orientation: Orientation::Horizontal,

            placement: Placement::new(5, 7),
        };
        let b4 = &Block {
            orientation: Orientation::Horizontal,

            placement: Placement::new(5, 6),
        };
        assert!(g.place_block(p1, p2, *b1));
        assert!(!g.does_intersect_existing_blocks(b2));
        assert!(!g.does_intersect_existing_blocks(b3));
        assert!(!g.does_intersect_existing_blocks(b4));
    }

    #[test]
    fn verify_cant_place_crossing_blocks() {
        struct TestCase(Block, Block);

        let tests = [
            TestCase(
                Block {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(0, 0),
                },
                Block {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(0, 0),
                },
            ),
            TestCase(
                Block {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(1, 1),
                },
                Block {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(1, 1),
                },
            ),
            TestCase(
                Block {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(3, 3),
                },
                Block {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(3, 3),
                },
            ),
        ];

        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);
        for t in tests.iter() {
            let mut grid = Grid::new();
            assert!(grid.place_block(&p1, &p2, t.0));
            assert!(!grid.place_block(&p1, &p2, t.1));
        }
    }

    #[test]
    fn verify_can_place_t_junctions() {
        struct TestCase(Block, Block);

        let tests = [
            TestCase(
                Block {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(0, 0),
                },
                Block {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(0, 1),
                },
            ),
            TestCase(
                Block {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(0, 0),
                },
                Block {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(1, 0),
                },
            ),
            TestCase(
                Block {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(7, 7),
                },
                Block {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(5, 6),
                },
            ),
            TestCase(
                Block {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(7, 7),
                },
                Block {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(5, 7),
                },
            ),
            TestCase(
                Block {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(7, 7),
                },
                Block {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(7, 6),
                },
            ),
            TestCase(
                Block::new(Placement::new(6, 3), Orientation::Horizontal),
                Block::new(Placement::new(4, 6), Orientation::Vertical),
            ),
        ];

        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);
        for t in tests {
            let mut grid = Grid::new();
            assert!(grid.place_block(&p1, &p2, t.0));
            assert!(
                grid.place_block(&p1, &p2, t.1),
                "existing {:?} and new {:?} intersect",
                t.0,
                t.1,
            );
        }
    }

    #[test]
    fn test_can_place_block() {
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);
        let mut grid = Grid::new();

        assert!(grid.can_place_block(
            &p1,
            &p2,
            Block::new(Placement { tile: 4, lane: 4 }, Orientation::Horizontal)
        ));

        assert!(grid.can_place_block(
            &p1,
            &p2,
            Block::new(Placement { tile: 4, lane: 7 }, Orientation::Horizontal)
        ));
        //
        // assert!(grid.can_place_block(
        //     &p1,
        //     &p2,
        //     Block::new(Placement { tile: 4, lane: 8 }, Orientation::Horizontal)
        // ));
        //
        assert!(!grid.can_place_block(
            &p1,
            &p2,
            Block::new(Placement { tile: 8, lane: 4 }, Orientation::Horizontal)
        ));
    }
}
