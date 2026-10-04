use crate::core::grid::Grid;
use crate::core::player::Player;
use crate::core::position::Position;
use crate::core::wall::Wall;

/// Which player goes first
pub const STARTING_PLAYER_ID: u8 = 0;

#[derive(Debug)]
pub struct Game {
    pub p1: Player,
    pub p2: Player,
    pub g: Grid,
    player_turn: u8,
}

#[derive(Debug)]
pub enum Move {
    PlaceWall(Wall),
    PlayerAdvance(Position),
}

impl Game {
    pub fn new(g: Grid, p1: Player, p2: Player) -> Self {
        Self {
            g,
            p1,
            p2,
            player_turn: STARTING_PLAYER_ID,
        }
    }

    pub fn can_add_wall(&mut self, b: Wall) -> bool {
        self.g.can_place_wall(&self.p1, &self.p2, b)
    }

    pub fn move_player(&mut self, pos: Position) {
        let player: &mut Player = if self.get_player_turn() == 0 {
            &mut self.p1
        } else {
            &mut self.p2
        };

        player.position = pos;
    }

    fn get_players(&self) -> (&Player, &Player) {
        if self.get_player_turn() == STARTING_PLAYER_ID {
            return (&self.p1, &self.p2);
        }

        (&self.p2, &self.p1)
    }

    pub fn get_player_options(&self) -> Vec<Position> {
        let (active_player, other_player) = self.get_players();

        // unoccupied tiles
        let player_unoccupied_tiles = self
            .g
            .get_unoccupied_reachable_neighbour_tiles(
                &active_player.position,
                &other_player.position,
            )
            .into_iter();

        // tiles behind the player
        let jump_over_tiles = self
            .g
            .get_unoccupied_reachable_neighbour_tiles(
                &other_player.position,
                &active_player.position,
            )
            .into_iter()
            .filter(|t| active_player.can_jump_over(other_player, &t.position));

        let cross_over_tiles = self
            .g
            .get_unoccupied_reachable_neighbour_tiles(
                &other_player.position,
                &active_player.position,
            )
            .into_iter()
            .filter(|t| {
                active_player.can_cross_over(other_player, &t.position)
            });
        //
        // let alternative_jumping_tiles = self
        //     .g
        //     .get_unoccupied_reachable_neighbour_tiles(
        //         &other_player.position,
        //         &active_player.position,
        //     )
        //     .into_iter()
        //     .filter(|t| {
        //         // test vertical and diagonal
        //         let mut diagonal = false;
        //         let mut can_jo = false;
        //         let mut can_co = false;
        //         if active_player.position.is_x_aligned(&t.position) {
        //             can_jo =
        //                 active_player.can_jump_over(other_player, &t.position);
        //             if !can_jo {
        //                 println!(
        //                     "adding diagonal {:?} because !can_jo: {:?} - {:?}",
        //                     t.position, active_player, other_player
        //                 );
        //                 diagonal = t
        //                     .position
        //                     .is_left_of(&other_player.position)
        //                     || other_player.position.is_left_of(&t.position);
        //             } else {
        //                 println!(
        //                     "ignoring position {:?} because can_jo",
        //                     t.position
        //                 );
        //             }
        //         }
        //
        //         if active_player.position.is_y_aligned(&t.position) {
        //             can_co =
        //                 active_player.can_cross_over(other_player, &t.position);
        //         }
        //
        //         can_jo || can_co || diagonal
        //     });

        // tiles around the player
        player_unoccupied_tiles
            .chain(cross_over_tiles)
            .chain(jump_over_tiles)
            .map(|t| Position::new(t.position.x, t.position.y))
            .collect()
    }

    pub fn is_position_occupied_by_player(&self, p: &Position) -> bool {
        self.p1.is_there(p) || self.p2.is_there(p)
    }

    pub fn place_wall(&mut self, b: Wall) -> bool {
        self.g.place_wall(&self.p1, &self.p2, b)
    }

    pub fn get_player_turn(&self) -> u8 {
        self.player_turn
    }

    pub fn next_player(&mut self) {
        self.player_turn ^= 1;
    }

    pub fn player_wall_counts(&self) -> (usize, usize) {
        (self.p1.remaining_walls, self.p2.remaining_walls)
    }

    pub fn process_move(&mut self, m: Move) -> bool {
        let curr_player: &mut Player = if self.player_turn == 0 {
            &mut self.p1
        } else {
            &mut self.p2
        };

        match m {
            Move::PlaceWall(wall) => {
                if !curr_player.has_remaining_walls() {
                    return false;
                }

                curr_player.consume_wall();
                self.place_wall(wall);
            }
            Move::PlayerAdvance(pos) => {
                let is_move_valid = self.get_player_options().contains(&pos);
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

    use crate::core::grid::Grid;
    use crate::core::orientation::Orientation;
    use crate::core::placement::Placement;
    use crate::core::player::{NUM_WALLS, StartingPosition};
    use crate::core::position::Position;
    use crate::core::wall::Wall;

    #[test]
    fn test_default() {
        let mut g = setup_new_game();
        assert!(g.can_add_wall(Wall {
            orientation: Orientation::Vertical,
            placement: Placement { tile: 0, lane: 0 },
        }));
        assert!(g.place_wall(Wall {
            orientation: Orientation::Vertical,
            placement: Placement { tile: 0, lane: 0 },
        }));
    }

    #[test]
    fn test_cant_add_wall_twice() {
        let mut g = setup_new_game();
        let b = Wall {
            orientation: Orientation::Vertical,
            placement: Placement { tile: 0, lane: 0 },
        };
        // test normal add
        assert!(g.can_add_wall(b));
        assert!(g.place_wall(b));
        //verify duplicate add rejected
        assert!(!g.can_add_wall(b));
        assert!(!g.place_wall(b));
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

        let b1 = Wall {
            orientation: Orientation::Vertical,
            placement: Placement { tile: 0, lane: 3 },
        };
        let b2 = Wall {
            orientation: Orientation::Vertical,
            placement: Placement { tile: 0, lane: 5 },
        };

        // bad wall
        let b3 = Wall {
            orientation: Orientation::Horizontal,
            placement: Placement { tile: 4, lane: 0 },
        };

        assert!(g.can_add_wall(b1));
        assert!(g.place_wall(b1));
        assert!(!g.can_add_wall(b1));

        assert!(g.can_add_wall(b2));
        assert!(g.place_wall(b2));
        assert!(!g.can_add_wall(b2));

        // this should fail
        assert!(!g.can_add_wall(b3));
        assert!(!g.place_wall(b3));
    }

    #[test]
    fn test_player_index_changed() {
        let mut g = setup_new_game();

        assert_eq!(STARTING_PLAYER_ID, g.player_turn);

        assert!(g.process_move(Move::PlaceWall(Wall::new(
            Placement::new(2, 1),
            Orientation::Horizontal,
        ))));

        assert_eq!(1, g.player_turn);

        assert!(g.process_move(Move::PlaceWall(Wall::new(
            Placement::new(2, 1),
            Orientation::Horizontal,
        ))));

        assert_eq!(STARTING_PLAYER_ID, g.player_turn);
    }

    #[test]
    fn test_player_wall_decrement() {
        let mut g = setup_new_game();

        let wall_counts = g.player_wall_counts();
        assert_eq!(NUM_WALLS, wall_counts.0);
        assert_eq!(NUM_WALLS, wall_counts.1);

        assert!(g.process_move(Move::PlaceWall(Wall::new(
            Placement::new(2, 1),
            Orientation::Horizontal,
        ))));

        let wall_counts = g.player_wall_counts();
        assert_eq!(NUM_WALLS - 1, wall_counts.0);
        assert_eq!(NUM_WALLS, wall_counts.1);

        assert!(g.process_move(Move::PlaceWall(Wall::new(
            Placement::new(2, 1),
            Orientation::Horizontal,
        ))));

        let wall_counts = g.player_wall_counts();
        assert_eq!(NUM_WALLS - 1, wall_counts.0);
        assert_eq!(NUM_WALLS - 1, wall_counts.1);
    }

    #[test]
    fn test_walls_exhausted() {
        let mut g = setup_new_game();
        let wall_counts = g.player_wall_counts();
        let mut total_wall_placed = 0;
        for col in 0..=4 {
            assert!(g.p1.has_remaining_walls(), "remaining: {}", wall_counts.0);
            assert!(g.p2.has_remaining_walls(), "remaining: {}", wall_counts.1);
            for i in [0, 2, 4, 6] {
                assert!(g.process_move(Move::PlaceWall(Wall::new(
                    Placement::new(i, col),
                    Orientation::Horizontal
                ))));
                total_wall_placed += 1;
            }
        }

        assert_eq!(20, total_wall_placed, "total {total_wall_placed}");

        assert!(!g.p1.has_remaining_walls(), "remaining: {}", wall_counts.0);
        assert!(!g.p2.has_remaining_walls(), "remaining: {}", wall_counts.1);

        assert!(!g.process_move(Move::PlaceWall(Wall::new(
            Placement::new(6, 6),
            Orientation::Horizontal
        ))));

        assert!(!g.p1.has_remaining_walls());
        assert!(!g.p2.has_remaining_walls());

        assert!(!g.process_move(Move::PlaceWall(Wall::new(
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
    fn test_get_player_options_with_walls() {
        let mut g = setup_new_game();
        g.place_wall(Wall::new(Placement::new(6, 6), Orientation::Horizontal));

        // player 1
        assert_eq!(DEFAULT_STARTING_NEIGHBOURS, g.get_player_options().len());

        // player 2
        g.next_player();
        assert_eq!(DEFAULT_STARTING_NEIGHBOURS, g.get_player_options().len());

        // new game - wall in from of the players
        let mut g = setup_new_game();
        g.place_wall(Wall::new(Placement::new(3, 0), Orientation::Horizontal));

        // player 1
        assert_eq!(
            DEFAULT_STARTING_NEIGHBOURS - 1,
            g.get_player_options().len()
        );

        g.place_wall(Wall::new(Placement::new(3, 7), Orientation::Horizontal));
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
    fn test_jump_over_player_no_walls_vertical() {
        let mut g = setup_new_game();
        g.p1.position = Position::new(4, 5);
        g.p2.position = Position::new(4, 6);

        let res = g.get_player_options();

        // TBLR
        let want = vec![
            Position { x: 4, y: 4 },
            Position { x: 3, y: 5 },
            Position { x: 5, y: 5 },
            // bottom added as extra square
            Position { x: 4, y: 7 },
        ];
        assert_eq!(want.len(), res.len(), "{:?}", res);
        assert_eq!(want, res, "{:?}", res);
    }

    #[test]
    fn test_no_jump_over_player_due_to_walls_vertical() {
        let mut g = setup_new_game();
        // top player
        g.p1.position = Position::new(4, 5);
        // bottom player
        g.p2.position = Position::new(4, 6);

        assert!(g.place_wall(Wall::new(
            Placement::new(4, 6),
            Orientation::Horizontal
        )));
        let res = g.get_player_options();

        // TBLR
        let want = vec![
            Position { x: 4, y: 4 },
            Position { x: 3, y: 5 },
            Position { x: 5, y: 5 },
        ];
        assert_eq!(0, g.get_player_turn());
        assert_eq!(want.len(), res.len(), "found: {:?}", res);
        assert_eq!(want, res, "{:?}", res);
    }

    #[test]
    fn test_no_other_player_due_to_wall_vertical() {
        let mut g = setup_new_game();
        // top player
        g.p1.position = Position::new(4, 5);
        // bottom player
        g.p2.position = Position::new(4, 6);

        // TBLR
        let want = vec![
            Position { x: 4, y: 7 },
            Position { x: 3, y: 6 },
            Position { x: 5, y: 6 },
        ];

        assert!(g.process_move(Move::PlaceWall(Wall::new(
            Placement::new(4, 4),
            Orientation::Horizontal
        ))));
        assert_eq!(1, g.get_player_turn());

        let res = g.get_player_options();
        assert_eq!(want.len(), res.len(), "found: {:?}", res);
        assert_eq!(want, res, "{:?}", res);
    }

    #[test]
    fn test_jump_over_player_no_walls_horizontal() {
        let mut g = setup_new_game();
        g.p1.position = Position::new(4, 5);
        g.p2.position = Position::new(5, 5);

        let res = g.get_player_options();

        // TBLR
        let want = vec![
            Position { x: 4, y: 4 },
            Position { x: 4, y: 6 },
            Position { x: 3, y: 5 },
            // right added as extra square
            Position { x: 6, y: 5 },
        ];
        assert_eq!(want.len(), res.len(), "{:?}", res);
        assert_eq!(want, res, "{:?}", res);
    }

    #[test]
    fn test_jump_over_player_no_walls_horizontal_right_to_left() {
        let mut g = setup_new_game();
        g.p1.position = Position::new(5, 5);
        g.p2.position = Position::new(4, 5);

        let res = g.get_player_options();

        // TBLR
        let want = vec![
            Position { x: 5, y: 4 },
            Position { x: 5, y: 6 },
            Position { x: 6, y: 5 },
            // left added as extra square
            Position { x: 3, y: 5 },
        ];
        assert_eq!(want.len(), res.len(), "{:?}", res);
        assert_eq!(want, res, "{:?}", res);
    }

    #[test]
    fn test_diagonals_vertically() {
        let mut g = setup_new_game();
        g.p1.position = Position::new(6, 1);
        g.p2.position = Position::new(6, 2);

        assert!(g.process_move(Move::PlaceWall(Wall::new(
            Placement::new(6, 0),
            Orientation::Horizontal,
        ))));

        let res = g.get_player_options();

        // (T)BLR
        let want = vec![
            Position { x: 6, y: 3 },
            Position { x: 5, y: 2 },
            Position { x: 7, y: 2 },
            // diagonals added as extra squares
            Position { x: 5, y: 1 },
            Position { x: 7, y: 1 },
        ];
        assert_eq!(want.len(), res.len(), "positions found: {:?}", res);
        assert_eq!(want, res, "{:?}", res);
    }

    #[test]
    fn test_diagonals_with_corner() {
        // TODO (dio) : test edges/boundaries

        let mut g = setup_new_game();
        g.p1.position = Position::new(6, 1);
        g.p2.position = Position::new(6, 2);

        assert!(g.process_move(Move::PlaceWall(Wall::new(
            Placement::new(6, 0),
            Orientation::Horizontal,
        ))));

        g.place_wall(Wall::new(Placement::new(1, 5), Orientation::Vertical));

        let res = g.get_player_options();

        // BR of TBLR
        let want = vec![
            Position { x: 6, y: 3 },
            Position { x: 7, y: 2 },
            // diagonals added as extra squares
            Position { x: 7, y: 1 },
        ];
        assert_eq!(want.len(), res.len(), "positions found: {:?}", res);
        assert_eq!(want, res, "{:?}", res);
    }
}
