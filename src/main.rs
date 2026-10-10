mod core;
mod ui;

use crate::core::game::GameState;
use core::game::{Game, Move, PLAYER_1_ID};
use core::player::{Player, StartingPosition};
use core::{grid, wall};
use macroquad::prelude::*;
use ui::utils::{
    MouseState, draw_game_grid_base, draw_game_stats, draw_move_preview,
    draw_placed_wall, draw_player, draw_wall_placement_preview,
    get_mouse_pointer_state, get_window_conf, highlight_player_options,
    pre_computed_grid_lines,
};

fn window_conf() -> Conf {
    get_window_conf(String::from("quoridor-rust"))
}

#[macroquad::main(window_conf)]
async fn main() {
    let grid = grid::Grid::new();

    let p1 = Player::new(String::from("p1"), StartingPosition::Top);
    let p2 = Player::new(String::from("p2"), StartingPosition::Bottom);
    let mut p1_score: usize = 0;
    let mut p2_score: usize = 0;

    let mut game = Game::new(grid, p1.clone(), p2.clone());
    loop {
        game_loop(&mut game).await;
        let winner = game.get_player_turn();
        if winner == PLAYER_1_ID {
            p1_score += 1;
        } else {
            p2_score += 1;
        }
        let grid = grid::Grid::new();
        game = Game::new(grid, p1.clone(), p2.clone());
    }
}

async fn game_loop(game: &mut Game) {
    let font = load_ttf_font("./assets/digital-7.ttf").await.unwrap();

    // this currently holds wall + user information
    // could wall be refactored to use this?
    let mut walls: Vec<wall::Wall> = Vec::with_capacity(20);

    // minor optimisation
    let grid_lines = pre_computed_grid_lines();

    loop {
        clear_background(GRAY);

        // draws borders, grid, padding and
        draw_game_grid_base(&grid_lines);

        draw_game_stats(Some(&font), game.player_wall_counts());

        // draw walls that have already been placed
        for b in &walls {
            draw_placed_wall(*b);
        }

        // draw players
        draw_player(&game.p1, game.get_player_turn() == PLAYER_1_ID);
        draw_player(&game.p2, game.get_player_turn() != PLAYER_1_ID);

        let res = get_mouse_pointer_state(mouse_position());
        let turn = game.get_player_turn();
        match &res {
            // hovering above a wall position
            MouseState::WallPosition(b) => {
                // hovering above a wall position
                if game.can_add_wall(*b) {
                    draw_wall_placement_preview(*b, turn);
                }

                if is_mouse_button_pressed(MouseButton::Left)
                    && game.can_add_wall(*b)
                {
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

                if is_mouse_button_pressed(MouseButton::Left) {
                    game.process_move(Move::PlayerAdvance(*pos));
                }
            }
            // anything else isn't drawable
            _ => {}
        }

        if game.state == GameState::GameFinished {
            return;
        }

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use crate::core::game::Game;
    use crate::core::grid;
    use crate::core::player::{Player, StartingPosition};
    use crate::game_loop;

    #[test]
    fn test_game_loop() {
        let grid = grid::Grid::new();

        let p1 = Player::new(String::from("p1"), StartingPosition::Top);
        let p2 = Player::new(String::from("p2"), StartingPosition::Bottom);

        let mut game = Game::new(grid, p1, p2);

        //     game_loop(&mut game).await;
    }
}
