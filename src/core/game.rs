use crate::core::block::Block;
use crate::core::grid::Grid;
use crate::core::player::Player;
use crate::core::position::Position;

/// Number of players for this game
const NUM_PLAYERS: u8 = 2;

/// Which player goes first
const DEFAULT_PLAYER_ORDER: u8 = 0;

#[derive(Debug)]
pub struct Game {
    pub p1: Player,
    pub p2: Player,
    pub g: Grid,
    player_turn: u8,
}

#[derive(Debug)]
pub enum Move {
    PlaceBlock(Block),
    PlayerAdvance(Position),
}

impl Game {
    pub fn new(g: Grid, p1: Player, p2: Player) -> Self {
        Self {
            g,
            p1,
            p2,
            player_turn: DEFAULT_PLAYER_ORDER,
        }
    }

    pub fn can_add_block(&mut self, b: Block) -> bool {
        self.g.can_place_block(&self.p1, &self.p2, b)
    }

    pub fn move_player(&mut self, pos: Position) {
        let player: &mut Player = if self.get_player_turn() == 0 {
            &mut self.p1
        } else {
            &mut self.p2
        };

        player.position = pos;
    }

    pub fn get_player_options(&self) -> Vec<Position> {
        let active_player: &Player = if self.get_player_turn() == 0 {
            &self.p1
        } else {
            &self.p2
        };

        let mut free_blocks = self
            .g
            .get_reachable_neighbour_tiles(&active_player.position)
            .into_iter()
            .filter(|t| !self.is_position_occupied_by_player(&t.position));

        let mut player_occupied_blocks = self
            .g
            .get_reachable_neighbour_tiles(&active_player.position)
            .into_iter()
            .filter(|t| self.is_position_occupied_by_player(&t.position));

        let positions = free_blocks
            .map(|t| Position::new(t.position.y, t.position.x))
            .collect();

        return positions;
    }

    pub fn is_position_occupied_by_player(&self, p: &Position) -> bool {
        (self.p1.position.x == p.x && self.p1.position.y == p.y)
            || (self.p2.position.x == p.x && self.p2.position.y == p.y)
    }

    pub fn place_block(&mut self, b: Block) -> bool {
        self.g.place_block(&self.p1, &self.p2, b)
    }

    pub fn get_player_turn(&self) -> u8 {
        self.player_turn
    }

    pub fn next_player(&mut self) {
        self.player_turn ^= 1;
    }

    pub fn player_block_counts(&self) -> (usize, usize) {
        (self.p1.remaining_blocks, self.p2.remaining_blocks)
    }

    pub fn process_move(&mut self, m: Move) -> bool {
        let curr_player: &mut Player = if self.player_turn == 0 {
            &mut self.p1
        } else {
            &mut self.p2
        };

        match m {
            Move::PlaceBlock(block) => {
                if !curr_player.has_remaining_blocks() {
                    return false;
                }

                curr_player.consume_block();
                self.place_block(block);
            }
            Move::PlayerAdvance(pos) => {
                let is_move_valid = self
                    .get_player_options()
                    .iter()
                    .any(|pp| pp.x == pos.y && pp.y == pos.x);
                if !is_move_valid {
                    return false;
                }
                self.move_player(pos);
            }
        }

        self.next_player();

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::core::block::Block;
    use crate::core::grid::Grid;
    use crate::core::orientation::Orientation;
    use crate::core::placement::Placement;
    use crate::core::player::{NUM_BLOCKS, StartingPosition};
    use crate::core::position::Position;
    use crate::core::tile::Tile;

    #[test]
    fn test_default() {
        let mut g = setup_new_game();
        assert!(g.can_add_block(Block {
            orientation: Orientation::Vertical,
            placement: Placement { tile: 0, lane: 0 },
        }));
        assert!(g.place_block(Block {
            orientation: Orientation::Vertical,
            placement: Placement { tile: 0, lane: 0 },
        }));
    }

    #[test]
    fn test_cant_add_block_twice() {
        let mut g = setup_new_game();
        let b = Block {
            orientation: Orientation::Vertical,
            placement: Placement { tile: 0, lane: 0 },
        };
        // test normal add
        assert!(g.can_add_block(b));
        assert!(g.place_block(b));
        //verify duplicate add rejected
        assert!(!g.can_add_block(b));
        assert!(!g.place_block(b));
    }

    #[test]
    fn test_is_position_occupied_by_player() {
        let mut g = setup_new_game();

        assert!(g.is_position_occupied_by_player(&Position { x: 4, y: 0 }));
        assert!(g.is_position_occupied_by_player(&Position { x: 4, y: 8 }));

        g.p1.position = Position { x: 4, y: 6 };
        assert!(g.is_position_occupied_by_player(&Position { x: 4, y: 6 }));
    }

    #[test]
    fn test_ui_bugs() {
        let mut g = setup_new_game();

        let b1 = Block {
            orientation: Orientation::Vertical,
            placement: Placement { tile: 0, lane: 3 },
        };
        let b2 = Block {
            orientation: Orientation::Vertical,
            placement: Placement { tile: 0, lane: 5 },
        };

        // bad block
        let b3 = Block {
            orientation: Orientation::Horizontal,
            placement: Placement { tile: 4, lane: 0 },
        };

        assert!(g.can_add_block(b1));
        assert!(g.place_block(b1));
        assert!(!g.can_add_block(b1));

        assert!(g.can_add_block(b2));
        assert!(g.place_block(b2));
        assert!(!g.can_add_block(b2));

        // this should fail
        assert!(!g.can_add_block(b3));
        assert!(!g.place_block(b3));

        let tile_vec = g.g.get_reachable_neighbour_tiles(&g.p1.position);
        assert_eq!(
            2,
            tile_vec.clone().len(),
            "{:?}",
            tile_vec.clone().iter().collect::<Vec<&&Tile>>(),
        );
    }

    #[test]
    fn test_player_index_changed() {
        let mut g = setup_new_game();

        assert_eq!(DEFAULT_PLAYER_ORDER, g.player_turn);

        assert!(g.process_move(Move::PlaceBlock(Block::new(
            Placement::new(2, 1),
            Orientation::Horizontal,
        ))));

        assert_eq!(1, g.player_turn);

        assert!(g.process_move(Move::PlaceBlock(Block::new(
            Placement::new(2, 1),
            Orientation::Horizontal,
        ))));

        assert_eq!(DEFAULT_PLAYER_ORDER, g.player_turn);
    }

    #[test]
    fn test_player_block_decrement() {
        let mut g = setup_new_game();

        let block_counts = g.player_block_counts();
        assert_eq!(NUM_BLOCKS, block_counts.0);
        assert_eq!(NUM_BLOCKS, block_counts.1);

        assert!(g.process_move(Move::PlaceBlock(Block::new(
            Placement::new(2, 1),
            Orientation::Horizontal,
        ))));

        let block_counts = g.player_block_counts();
        assert_eq!(NUM_BLOCKS - 1, block_counts.0);
        assert_eq!(NUM_BLOCKS, block_counts.1);

        assert!(g.process_move(Move::PlaceBlock(Block::new(
            Placement::new(2, 1),
            Orientation::Horizontal,
        ))));

        let block_counts = g.player_block_counts();
        assert_eq!(NUM_BLOCKS - 1, block_counts.0);
        assert_eq!(NUM_BLOCKS - 1, block_counts.1);
    }

    #[test]
    fn test_blocks_exhausted() {
        let mut g = setup_new_game();
        let block_counts = g.player_block_counts();
        let mut total_block_placed = 0;
        for col in 0..=4 {
            assert!(
                g.p1.has_remaining_blocks(),
                "remaining blocks: {}",
                block_counts.0
            );
            assert!(
                g.p2.has_remaining_blocks(),
                "remaining blocks: {}",
                block_counts.1
            );
            for i in [0, 2, 4, 6] {
                assert!(g.process_move(Move::PlaceBlock(Block::new(
                    Placement::new(i, col),
                    Orientation::Horizontal
                ))));
                total_block_placed += 1;
            }
        }

        assert_eq!(
            20, total_block_placed,
            "total_block_placed {total_block_placed}"
        );

        assert!(
            !g.p1.has_remaining_blocks(),
            "remaining blocks: {}",
            block_counts.0
        );
        assert!(
            !g.p2.has_remaining_blocks(),
            "remaining blocks: {}",
            block_counts.1
        );

        assert!(!g.process_move(Move::PlaceBlock(Block::new(
            Placement::new(6, 6),
            Orientation::Horizontal
        ))));

        assert!(!g.p1.has_remaining_blocks());
        assert!(!g.p2.has_remaining_blocks());

        assert!(!g.process_move(Move::PlaceBlock(Block::new(
            Placement::new(6, 6),
            Orientation::Horizontal
        ))));
    }

    fn setup_new_game() -> Game {
        let grid = Grid::new();
        let p1: Player = Player::new(String::from("a"), StartingPosition::Top);
        let p2: Player =
            Player::new(String::from("b"), StartingPosition::Bottom);
        Game::new(grid, p1, p2)
    }
    const DEFAULT_STARTING_NEIGHBOURS: usize = 3;

    #[test]
    fn test_get_player_options() {
        let mut g = setup_new_game();

        // player 1
        assert_eq!(DEFAULT_STARTING_NEIGHBOURS, g.get_player_options().len());

        // player 2
        g.next_player();
        assert_eq!(DEFAULT_STARTING_NEIGHBOURS, g.get_player_options().len());
    }

    #[test]
    fn test_get_player_options_with_blocks() {
        // new game - random block in the middle of the board
        let mut g = setup_new_game();
        g.place_block(Block::new(
            Placement::new(6, 6),
            Orientation::Horizontal,
        ));

        // player 1
        assert_eq!(DEFAULT_STARTING_NEIGHBOURS, g.get_player_options().len());

        // player 2
        g.next_player();
        assert_eq!(DEFAULT_STARTING_NEIGHBOURS, g.get_player_options().len());

        // new game - blocks in from of the players
        let mut g = setup_new_game();
        g.place_block(Block::new(
            Placement::new(3, 0),
            Orientation::Horizontal,
        ));

        // player 1
        assert_eq!(
            DEFAULT_STARTING_NEIGHBOURS - 1,
            g.get_player_options().len()
        );

        g.place_block(Block::new(
            Placement::new(3, 7),
            Orientation::Horizontal,
        ));
        // player 2
        g.next_player();
        assert_eq!(
            DEFAULT_STARTING_NEIGHBOURS - 1,
            g.get_player_options().len()
        );
    }

    #[test]
    fn test_process_illegal_move_far_away() {
        let mut g = setup_new_game();

        // illegal moves
        assert!(!g.process_move(Move::PlayerAdvance(Position::new(0, 0))));
        assert!(!g.process_move(Move::PlayerAdvance(Position::new(0, 1))));

        // player 1 move forward
        assert!(g.process_move(Move::PlayerAdvance(Position::new(4, 1))));

        // player 2 move forward
        assert!(
            g.process_move(Move::PlayerAdvance(Position::new(4, 7))),
            "player turn {}, {:?}",
            g.get_player_turn(),
            g.p2,
        );
    }

    #[test]
    fn test_jump_over_player_no_blocks() {
        let mut g = setup_new_game();
        g.p1.position = Position::new(5, 5);
        g.p2.position = Position::new(5, 6);

        let options = g.get_player_options().len();
        assert_eq!(4, options, "{:?}", options);
    }
}
