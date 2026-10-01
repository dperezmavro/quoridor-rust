mod core;
mod ui;

use crate::ui::utils::{
    draw_block_placement_preview, draw_game_stats, draw_move_preview,
    get_window_conf,
};
use core::game::{Game, Move};
use core::player::{Player, StartingPosition};
use core::{block, grid};
use macroquad::prelude::*;
use ui::utils::{
    MouseState, draw_game_grid_base, draw_player, get_mouse_pointer_state,
    highlight_player_options, pre_computed_grid_lines, draw_placed_block,
};

fn window_conf() -> Conf {
    get_window_conf(String::from("quoridor-rust"))
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_ttf_font("./assets/digital-7.ttf").await.unwrap();

    // this currently holds block + user information
    // could block be refactored to use this?
    let mut blocks: Vec<block::Block> = Vec::with_capacity(20);

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

        draw_game_stats(Some(&font), game.player_block_counts());

        // draw blocks that have already been placed
        for b in &blocks {
            draw_placed_block(*b);
        }

        // draw players
        draw_player(&game.p1);
        draw_player(&game.p2);

        let res = get_mouse_pointer_state();
        match &res {
            // hovering above a block position
            MouseState::BlockPosition(b) => {
                let turn = game.get_player_turn();
                // hovering above a block position
                if game.can_add_block(*b) {
                    draw_block_placement_preview(*b, turn);
                }

                if is_mouse_button_pressed(MouseButton::Left)
                    && game.can_add_block(*b)
                {
                    game.process_move(Move::PlaceBlock(*b));
                    blocks.push(*b);
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
