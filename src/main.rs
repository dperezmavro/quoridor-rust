mod core;
mod ui;

use crate::core::game::GameState;
use crate::ui::utils::Ui;
use core::game::{Game, Move, PLAYER_1_ID};
use core::player::{Player, StartingPosition};
use core::{grid, wall};
use macroquad::prelude::Conf;
use ui::utils::{
    MouseState, draw_move_preview, draw_placed_wall,
    draw_wall_placement_preview, get_mouse_pointer_state, get_window_conf,
    highlight_player_options,
};

fn window_conf() -> Conf {
    get_window_conf(String::from("quoridor-rust"))
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut p1_score: usize = 0;
    let mut p2_score: usize = 0;

    loop {
        let mut game = new_game();
        let ui: Ui = Ui::new().await;
        game_loop(&mut game, ui).await;
        let winner_id = game.get_player_turn();
        if winner_id == PLAYER_1_ID {
            p1_score += 1;
        } else {
            p2_score += 1;
        }

        println!(
            "Player {winner_id} wins this round! Scores: P1 {p1_score}, P2 {p2_score}"
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

trait UIHandler {
    async fn new() -> Self;
    fn draw_game_base(&self, remaining_walls: (usize, usize));

    fn mouse_position(&self) -> (f32, f32);

    fn is_mouse_button_pressed(&self) -> bool;

    async fn next_frame(&self);

    fn draw_player(&self, p: &Player, highlight: bool);
}
async fn game_loop<T>(game: &mut Game, ui: T)
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
            draw_placed_wall(*b);
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
                    draw_wall_placement_preview(*b, turn);
                }

                if ui.is_mouse_button_pressed() && game.can_add_wall(*b) {
                    game.process_move(Move::PlaceWall(*b));
                    walls.push(*b);
                }
            }
            MouseState::RowCol(pos) => {
                let player_options = game.get_player_options();
                highlight_player_options(turn, &player_options);

                if player_options.contains(pos)
                    && !game.is_position_occupied_by_player(pos)
                {
                    draw_move_preview(pos);
                }

                if ui.is_mouse_button_pressed() {
                    game.process_move(Move::PlayerAdvance(*pos));
                }
            }
            // anything else isn't drawable
            _ => {}
        }

        if game.state == GameState::GameFinished {
            return;
        }

        ui.next_frame().await
    }
}

#[cfg(test)]
mod tests {

    struct MockUi {}

    impl UIHandler for MockUi {
        async fn new() -> Self {
            Self {}
        }

        fn draw_game_base(&self, _remaining_walls: (usize, usize)) {}

        fn mouse_position(&self) -> (f32, f32) {
            println!("in mouse_position");

            (0.0, 0.0)
        }

        fn is_mouse_button_pressed(&self) -> bool {
            println!("in mouse button_pressed");
            false
        }

        async fn next_frame(&self) {}

        fn draw_player(&self, p: &Player, highlight: bool){}
    }

    use super::*;
    use tokio;
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_game_loop() {
        let mut g = new_game();
        let mui = MockUi::new().await;
        // game_loop(&mut g, mui).await;
    }
}
