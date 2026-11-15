mod cli;
mod core;
mod game_launcher;
mod save_file;
mod ui;

use crate::core::game::GameState;
use crate::core::game::Move;
use crate::core::player::NUM_WALLS;
use crate::core::player_turn::PLAYER_1_ID;
use crate::core::wall::Wall;
use crate::game_launcher::GameLauncher;
use crate::save_file::SaveFileManager;
use crate::ui::ui_handler::UIHandler;
use crate::ui::utils::Ui;
use crate::ui::utils::{
    MouseState, draw_move_preview, draw_placed_wall,
    draw_wall_placement_preview, get_mouse_pointer_state,
    highlight_player_options, window_conf,
};
use core::game_rounds_state::GameRoundsState;

use std::cell::RefCell;
use std::rc::Rc;

#[macroquad::main(window_conf)]
async fn main() {
    let q_cli = cli::parse();
    let ui: Ui = Ui::new().await;
    let sfm = SaveFileManager::new();
    let mut gl = GameLauncher::new(q_cli, ui, sfm);
    gl.setup();

    loop {
        gl.run().await;
    }
}

/// Runs the game loop for a single game
async fn game_loop<T>(
    grs: Rc<RefCell<GameRoundsState>>,
    ui: &T,
    scores: (usize, usize),
) -> u8
where
    T: UIHandler,
{
    // this currently holds wall + user information
    // could wall be refactored to use this?
    let mut walls: Vec<Wall> = Vec::with_capacity(2 * NUM_WALLS);
    loop {
        ui.draw_game_base(grs.borrow().game.player_wall_counts(), scores);

        // draw walls that have already been placed
        for b in &walls {
            draw_placed_wall(ui, *b);
        }

        // draw players
        ui.draw_player(
            &grs.borrow().game.p1,
            grs.borrow().game.get_player_turn() == PLAYER_1_ID,
        );
        ui.draw_player(
            &grs.borrow().game.p2,
            grs.borrow().game.get_player_turn() != PLAYER_1_ID,
        );

        let res = get_mouse_pointer_state(ui.mouse_position());
        let turn = grs.borrow().game.get_player_turn();
        match &res {
            // hovering above a wall position
            MouseState::WallPosition(b) => {
                // hovering above a wall position
                if grs.borrow_mut().game.can_add_wall(*b) {
                    draw_wall_placement_preview(ui, *b, turn);
                }

                if ui.is_mouse_button_pressed()
                    && grs.borrow_mut().game.can_add_wall(*b)
                {
                    let wall_placed =
                        grs.borrow_mut().game.process_move(Move::PlaceWall(*b));
                    if wall_placed {
                        walls.push(*b);
                    }
                }
            }
            MouseState::RowCol(pos) => {
                let player_options = grs.borrow().game.get_player_options();
                highlight_player_options(ui, turn, &player_options);

                if player_options.contains(pos)
                    && !grs.borrow().game.is_position_occupied_by_player(pos)
                {
                    draw_move_preview(ui, pos);
                }

                if ui.is_mouse_button_pressed() {
                    grs.borrow_mut()
                        .game
                        .process_move(Move::PlayerAdvance(*pos));
                }
            }
            // anything else isn't drawable
            _ => {}
        }

        ui.next_frame().await;
        if let GameState::GameFinished(player_turn) = grs.borrow().game.state {
            return player_turn;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::player::Player;
    use crate::core::player_turn::PLAYER_2_ID;
    use macroquad::color::Color;
    use macroquad::prelude::{TextParams, Vec2};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::option::Option;

    struct MockUiHandler {
        click_coordinates: RefCell<VecDeque<(f32, f32)>>,
    }

    impl UIHandler for MockUiHandler {
        async fn new() -> Self {
            Self {
                click_coordinates: RefCell::new(VecDeque::with_capacity(10)),
            }
        }

        fn draw_game_base(
            &self,
            _remaining_walls: (usize, usize),
            _scores: (usize, usize),
        ) {
        }

        fn mouse_position(&self) -> (f32, f32) {
            self.click_coordinates.borrow_mut().pop_front().unwrap()
        }

        fn is_mouse_button_pressed(&self) -> bool {
            true
        }

        async fn next_frame(&self) {}

        fn draw_player(&self, _p: &Player, _highlight: bool) {}

        fn draw_rectangle(
            &self,
            _x: f32,
            _y: f32,
            _w: f32,
            _h: f32,
            _color: Color,
        ) {
        }

        fn draw_circle(&self, _x: f32, _y: f32, _r: f32, _color: Color) {}

        fn draw_text_ex(
            &self,
            _text: impl AsRef<str>,
            _x: f32,
            _y: f32,
            _params: TextParams,
        ) {
        }

        fn get_text_center(
            &self,
            _text: impl AsRef<str>,
            _font: Option<&macroquad::text::Font>,
            _font_size: u16,
            _font_scale: f32,
            _rotation: f32,
        ) -> macroquad::math::Vec2 {
            Vec2 { x: 0.0, y: 0.0 }
        }
    }

    const TILE_COORD_0: f32 = 75.0;
    const TILE_COORD_1: f32 = 150.0;
    const TILE_COORD_2: f32 = 230.0;
    const TILE_COORD_3: f32 = 320.0;
    const TILE_COORD_4: f32 = 375.0;
    const TILE_COORD_5: f32 = 460.0;
    const TILE_COORD_6: f32 = 520.0;
    const TILE_COORD_7: f32 = 600.0;

    const WALL_LANE_0_COORD: f32 = 120.0;
    const WALL_LANE_1_COORD: f32 = 190.0;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_straight_move_no_walls() {
        let gg = GameRoundsState::new();

        let mut mui = MockUiHandler::new().await;
        let click_coordinates = RefCell::new(VecDeque::from([
            (TILE_COORD_4, TILE_COORD_1), // p1 - y = 1
            (TILE_COORD_4, TILE_COORD_7), // p2 - y = 7
            (TILE_COORD_4, TILE_COORD_2), // p1 - y = 2
            (TILE_COORD_4, TILE_COORD_6), // p2 - y = 6
            (TILE_COORD_4, TILE_COORD_3), // p1 - y = 3
            (TILE_COORD_4, TILE_COORD_5), // p2 - y = 5
            (TILE_COORD_4, TILE_COORD_4), // p1 - y = 4
            (TILE_COORD_4, TILE_COORD_3), // p2 - y = 3
            (TILE_COORD_4, TILE_COORD_5), // p1 - y = 5
            (TILE_COORD_4, TILE_COORD_2), // p2 - y = 2
            (TILE_COORD_4, TILE_COORD_6), // p1 - y = 6
            (TILE_COORD_4, TILE_COORD_1), // p2 - y = 1
            (TILE_COORD_4, TILE_COORD_7), // p1 - y = 7
            (TILE_COORD_4, TILE_COORD_0), // p2 - y = 0
        ]));
        mui.click_coordinates = click_coordinates;
        assert_eq!(
            PLAYER_2_ID,
            game_loop(Rc::new(RefCell::new(gg)), &mui, (0, 0)).await
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_place_walls() {
        let gg = GameRoundsState::new();

        let mut mui = MockUiHandler::new().await;
        let click_coordinates = RefCell::new(VecDeque::from([
            (WALL_LANE_0_COORD, TILE_COORD_0), // p1 place wall vertical in 0,0
            (WALL_LANE_1_COORD, TILE_COORD_2), // p1 place wall vertical in 1,2
            (TILE_COORD_4, TILE_COORD_1),      // p1 - y = 1
            (TILE_COORD_4, TILE_COORD_7),      // p2 - y = 7
            (TILE_COORD_4, TILE_COORD_2),      // p1 - y = 2
            (TILE_COORD_4, TILE_COORD_6),      // p2 - y = 6
            (TILE_COORD_4, TILE_COORD_3),      // p1 - y = 3
            (TILE_COORD_4, TILE_COORD_5),      // p2 - y = 5
            (TILE_COORD_4, TILE_COORD_4),      // p1 - y = 4
            (TILE_COORD_4, TILE_COORD_3),      // p2 - y = 3
            (TILE_COORD_4, TILE_COORD_5),      // p1 - y = 5
            (TILE_COORD_4, TILE_COORD_2),      // p2 - y = 2
            (TILE_COORD_4, TILE_COORD_6),      // p1 - y = 6
            (TILE_COORD_4, TILE_COORD_1),      // p2 - y = 1
            (TILE_COORD_4, TILE_COORD_7),      // p1 - y = 7
            (TILE_COORD_4, TILE_COORD_0),      // p2 - y = 0
        ]));
        mui.click_coordinates = click_coordinates;
        assert_eq!(
            PLAYER_2_ID,
            game_loop(Rc::new(RefCell::new(gg)), &mui, (0, 0)).await
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_random_click() {
        let gg = GameRoundsState::new();

        let mut mui = MockUiHandler::new().await;
        let click_coordinates = RefCell::new(VecDeque::from([
            (10.0, 10.0),                 // random click - p1
            (2.5, 120.5),                 // random click - p1
            (TILE_COORD_4, TILE_COORD_1), // p1 - y = 1
            (TILE_COORD_4, TILE_COORD_7), // p2 - y = 7
            (TILE_COORD_4, TILE_COORD_2), // p1 - y = 2
            (TILE_COORD_4, TILE_COORD_6), // p2 - y = 6
            (TILE_COORD_4, TILE_COORD_3), // p1 - y = 3
            (150.0, 850.0),               // random click p2
            (TILE_COORD_4, TILE_COORD_5), // p2 - y = 5
            (TILE_COORD_4, TILE_COORD_4), // p1 - y = 4
            (TILE_COORD_4, TILE_COORD_3), // p2 - y = 3
            (TILE_COORD_4, TILE_COORD_5), // p1 - y = 5
            (TILE_COORD_4, TILE_COORD_2), // p2 - y = 2
            (TILE_COORD_4, TILE_COORD_6), // p1 - y = 6
            (TILE_COORD_4, TILE_COORD_1), // p2 - y = 1
            (TILE_COORD_4, TILE_COORD_7), // p1 - y = 7
            (TILE_COORD_4, TILE_COORD_0), // p2 - y = 0
        ]));
        mui.click_coordinates = click_coordinates;
        assert_eq!(
            PLAYER_2_ID,
            game_loop(Rc::new(RefCell::new(gg)), &mui, (0, 0)).await
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_game_setup_straight_move_no_walls() {
        let mut mui = MockUiHandler::new().await;
        let click_coordinates = RefCell::new(VecDeque::from([
            (TILE_COORD_4, TILE_COORD_1), // p1 - y = 1
            (TILE_COORD_4, TILE_COORD_7), // p2 - y = 7
            (TILE_COORD_4, TILE_COORD_2), // p1 - y = 2
            (TILE_COORD_4, TILE_COORD_6), // p2 - y = 6
            (TILE_COORD_4, TILE_COORD_3), // p1 - y = 3
            (TILE_COORD_4, TILE_COORD_5), // p2 - y = 5
            (TILE_COORD_4, TILE_COORD_4), // p1 - y = 4
            (TILE_COORD_4, TILE_COORD_3), // p2 - y = 3
            (TILE_COORD_4, TILE_COORD_5), // p1 - y = 5
            (TILE_COORD_4, TILE_COORD_2), // p2 - y = 2
            (TILE_COORD_4, TILE_COORD_6), // p1 - y = 6
            (TILE_COORD_4, TILE_COORD_1), // p2 - y = 1
            (TILE_COORD_4, TILE_COORD_7), // p1 - y = 7
            (TILE_COORD_4, TILE_COORD_0), // p2 - y = 0
        ]));
        mui.click_coordinates = click_coordinates;

        let q_cli = crate::cli::QuoridorCli {
            save_file: None,
            load_file: None,
        };

        let sfm = SaveFileManager::new();
        let mut gl = GameLauncher::new(q_cli, mui, sfm);
        gl.setup();
        gl.run().await;
    }
}
