mod core;
mod ui;

use core::game::{DEFAULT_PLAYER_ORDER, Game, Move};
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
    let font = load_ttf_font("./assets/digital-7.ttf").await.unwrap();

    // this currently holds wall + user information
    // could wall be refactored to use this?
    let mut walls: Vec<wall::Wall> = Vec::with_capacity(20);

    // minor optimisation
    let pre_computed_grid_lines = pre_computed_grid_lines();

    let grid = grid::Grid::new();
    let p1 = Player::new(String::from("p1"), StartingPosition::Top);
    let p2 = Player::new(String::from("p2"), StartingPosition::Bottom);

    let mut game = Game::new(grid, p1, p2);

    loop {
        clear_background(GRAY);

        // draws borders, grid, padding and
        draw_game_grid_base(&pre_computed_grid_lines);

        draw_game_stats(Some(&font), game.player_wall_counts());

        // draw walls that have already been placed
        for b in &walls {
            draw_placed_wall(*b);
        }

        // draw players
        draw_player(&game.p1, game.get_player_turn() == DEFAULT_PLAYER_ORDER);
        draw_player(&game.p2, game.get_player_turn() != DEFAULT_PLAYER_ORDER);

        let res = get_mouse_pointer_state(mouse_position());
        match &res {
            // hovering above a wall position
            MouseState::WallPosition(b) => {
                let turn = game.get_player_turn();
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
                let available_move_tiles_for_player = game.get_player_options();
                highlight_player_options(
                    game.get_player_turn(),
                    &available_move_tiles_for_player,
                );

                if available_move_tiles_for_player.contains(pos)
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

        next_frame().await
    }
}
