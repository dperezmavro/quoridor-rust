mod core;
mod ui;

use crate::core::game::GameState;
use crate::ui::utils::Ui;
use core::game::{Game, Move, PLAYER_1_ID};
use core::player::{Player, StartingPosition};
use core::{grid, wall};
use macroquad::prelude::Conf;
use ui::ui_handler::UIHandler;
use ui::utils::{
    MouseState, draw_move_preview, draw_placed_wall,
    draw_wall_placement_preview, get_mouse_pointer_state, get_window_conf,
    highlight_player_options,
};

fn window_conf() -> Conf {
    get_window_conf(String::from("quoridor-rust"))
}

#[derive(Default)]
struct Games {
    total_rounds: usize,
    p1_score: usize,
    p2_score: usize,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut gg = Games::default();

    loop {
        gg.total_rounds += 1;
        let mut game = new_game();
        let ui: Ui = Ui::new().await;
        let winner_id = game_loop(&mut game, ui).await;
        if winner_id == PLAYER_1_ID {
            gg.p1_score += 1;
        } else {
            gg.p2_score += 1;
        }

        println!(
            "Player {winner_id} wins this round! Scores: P1 {}, P2 {}",
            gg.p1_score, gg.p2_score
        );
        game = new_game();
    }
}

fn new_game() -> Game {
    let grid = grid::Grid::new();

    let p1 = Player::new(String::from("p1"), StartingPosition::Top);
    let p2 = Player::new(String::from("p2"), StartingPosition::Bottom);
    Game::new(grid, p1, p2)
}

async fn game_loop<T>(game: &mut Game, ui: T) -> u8
where
    T: UIHandler,
{
    // this currently holds wall + user information
    // could wall be refactored to use this?
    let mut walls: Vec<wall::Wall> = Vec::with_capacity(20);

    loop {
        ui.draw_game_base(game.player_wall_counts());

        // draw walls that have already been placed
        for b in &walls {
            draw_placed_wall(&ui, *b);
        }

        // draw players
        ui.draw_player(&game.p1, game.get_player_turn() == PLAYER_1_ID);
        ui.draw_player(&game.p2, game.get_player_turn() != PLAYER_1_ID);

        let res = get_mouse_pointer_state(ui.mouse_position());
        let turn = game.get_player_turn();
        match &res {
            // hovering above a wall position
            MouseState::WallPosition(b) => {
                // hovering above a wall position
                if game.can_add_wall(*b) {
                    draw_wall_placement_preview(&ui, *b, turn);
                }

                if ui.is_mouse_button_pressed() && game.can_add_wall(*b) {
                    game.process_move(Move::PlaceWall(*b));
                    walls.push(*b);
                }
            }
            MouseState::RowCol(pos) => {
                let player_options = game.get_player_options();
                highlight_player_options(&ui, turn, &player_options);

                if player_options.contains(pos)
                    && !game.is_position_occupied_by_player(pos)
                {
                    draw_move_preview(&ui, pos);
                }

                if ui.is_mouse_button_pressed() {
                    game.process_move(Move::PlayerAdvance(*pos));
                }
            }
            // anything else isn't drawable
            _ => {}
        }

        if let GameState::GameFinished(player_turn) = game.state {
            return player_turn;
        }

        ui.next_frame().await
    }
}

#[cfg(test)]
mod tests {

    struct MockUiHandler {
        click_coordinates: RefCell<VecDeque<(f32, f32)>>,
    }

    impl UIHandler for MockUiHandler {
        async fn new() -> Self {
            Self {
                click_coordinates: RefCell::new(VecDeque::with_capacity(10)),
            }
        }

        fn draw_game_base(&self, _remaining_walls: (usize, usize)) {}

        fn mouse_position(&self) -> (f32, f32) {
            // println!("in mouse_position");
            self.click_coordinates.borrow_mut().pop_front().unwrap()
        }

        fn is_mouse_button_pressed(&self) -> bool {
            // println!("in mouse button_pressed");
            true
        }

        async fn next_frame(&self) {}

        fn draw_player(&self, _p: &Player, _highlight: bool) {
            // println!("in mouse button_pressed");
        }

        fn draw_rectangle(
            &self,
            _x: f32,
            _y: f32,
            _w: f32,
            _h: f32,
            _color: Color,
        ) {
            // println!("in draw_rectangle");
        }

        fn draw_circle(&self, _x: f32, _y: f32, _r: f32, _color: Color) {
            println!("in draw_circle");
        }
    }

    use super::*;
    use macroquad::color::Color;
    use std::cell::RefCell;
    use std::collections::VecDeque;

    const TILE_IDX_0: f32 = 75.0;
    const TILE_IDX_1: f32 = 150.0;
    const TILE_IDX_2: f32 = 230.0;
    const TILE_IDX_3: f32 = 320.0;
    const TILE_IDX_4: f32 = 375.0;
    const TILE_IDX_5: f32 = 460.0;
    const TILE_IDX_6: f32 = 520.0;
    const TILE_IDX_7: f32 = 600.0;

    const WALL_LANE_0: f32 = 120.0;
    const WALL_LANE_1: f32 = 190.0;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_staight_move_no_walls() {
        let mut g = new_game();
        let mut mui = MockUiHandler::new().await;
        let click_coordinates = RefCell::new(VecDeque::from([
            (TILE_IDX_4, TILE_IDX_1), // p1 - y = 1
            (TILE_IDX_4, TILE_IDX_7), // p2 - y = 7
            (TILE_IDX_4, TILE_IDX_2), // p1 - y = 2
            (TILE_IDX_4, TILE_IDX_6), // p2 - y = 6
            (TILE_IDX_4, TILE_IDX_3), // p1 - y = 3
            (TILE_IDX_4, TILE_IDX_5), // p2 - y = 5
            (TILE_IDX_4, TILE_IDX_4), // p1 - y = 4
            (TILE_IDX_4, TILE_IDX_3), // p2 - y = 3
            (TILE_IDX_4, TILE_IDX_5), // p1 - y = 5
            (TILE_IDX_4, TILE_IDX_2), // p2 - y = 2
            (TILE_IDX_4, TILE_IDX_6), // p1 - y = 6
            (TILE_IDX_4, TILE_IDX_1), // p2 - y = 1
            (TILE_IDX_4, TILE_IDX_7), // p1 - y = 7
            (TILE_IDX_4, TILE_IDX_0), // p2 - y = 0
        ]));
        mui.click_coordinates = click_coordinates;
        assert_eq!(1, game_loop(&mut g, mui).await);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_place_walls() {
        let mut g = new_game();

        let mut mui = MockUiHandler::new().await;
        let click_coordinates = RefCell::new(VecDeque::from([
            (WALL_LANE_0, TILE_IDX_0), // p1 place wall vertical in 0,0
            (WALL_LANE_0, TILE_IDX_2), // p1 place wall vertical in 0,2
            (TILE_IDX_4, TILE_IDX_1),  // p1 - y = 1
            (TILE_IDX_4, TILE_IDX_7),  // p2 - y = 7
            (TILE_IDX_4, TILE_IDX_2),  // p1 - y = 2
            (TILE_IDX_4, TILE_IDX_6),  // p2 - y = 6
            (TILE_IDX_4, TILE_IDX_3),  // p1 - y = 3
            (TILE_IDX_4, TILE_IDX_5),  // p2 - y = 5
            (TILE_IDX_4, TILE_IDX_4),  // p1 - y = 4
            (TILE_IDX_4, TILE_IDX_3),  // p2 - y = 3
            (TILE_IDX_4, TILE_IDX_5),  // p1 - y = 5
            (TILE_IDX_4, TILE_IDX_2),  // p2 - y = 2
            (TILE_IDX_4, TILE_IDX_6),  // p1 - y = 6
            (TILE_IDX_4, TILE_IDX_1),  // p2 - y = 1
            (TILE_IDX_4, TILE_IDX_7),  // p1 - y = 7
            (TILE_IDX_4, TILE_IDX_0),  // p2 - y = 0
        ]));
        mui.click_coordinates = click_coordinates;
        assert_eq!(1, game_loop(&mut g, mui).await);
    }
}
