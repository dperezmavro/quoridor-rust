mod core;
mod ui;

use crate::core::position::Position;
use crate::ui::utils::{
    draw_block_placement_preview, draw_game_stats, draw_game_tile_preview,
    get_window_conf,
};
use core::game::{Game, Move};
use core::player::{Player, StartingPosition};
use core::{block, grid};
use macroquad::prelude::*;
use ui::utils::{
    MouseState, draw_game_grid_base, draw_player, get_mouse_pointer_state,
    highlight_player_options, pre_computed_grid_lines,
};

fn window_conf() -> Conf {
    get_window_conf(String::from("quoridor-rust"))
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_ttf_font("./assets/digital-7.ttf").await.unwrap();

    let mut blocks: Vec<(block::Block, u8)> = Vec::with_capacity(20);
    let pre_computed_grid_lines = pre_computed_grid_lines();

    let grid = grid::Grid::new();
    let p1 = Player::new(String::from("p1"), StartingPosition::Top);
    let p2 = Player::new(String::from("p2"), StartingPosition::Bottom);

    let mut game = Game::new(grid, p1, p2);

    loop {
        clear_background(GRAY);

        // draws borders, grid, padding and
        draw_game_grid_base(&pre_computed_grid_lines);

        draw_game_stats(
            Some(&font),
            game.p1.remaining_blocks,
            game.p2.remaining_blocks,
        );

        // draw blocks that have already been placed
        for b in &blocks {
            // temporarily using this function
            draw_block_placement_preview(b.0, b.1);
        }

        // draw players
        draw_player(game.p1.clone());
        draw_player(game.p2.clone());

        let available_move_tiles_for_player = game.get_player_options();
        highlight_player_options(
            game.get_player_turn(),
            &available_move_tiles_for_player,
        );

        let res = get_mouse_pointer_state();
        match &res {
            // hovering above a block position
            MouseState::BlockPosition(b) => {
                let turn = game.get_player_turn();
                // hovering above a block position
                if game.can_add_block(b.clone()) {
                    draw_block_placement_preview(b.clone(), turn);
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    if game.can_add_block(b.clone()) {
                        game.process_move(Move::PlaceBlock(b.clone()));
                        blocks.push((b.clone(), turn));
                    }
                }
            }
            MouseState::RowCol(position) => {
                let p = &Position {
                    x: position.y,
                    y: position.x,
                };

                if available_move_tiles_for_player
                    .iter()
                    .any(|pp| pp.x == p.y && pp.y == p.x)
                {
                    if !game.is_position_occupied_by_player(p) {
                        draw_game_tile_preview(position);
                    }
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    game.move_player(p.clone());
                }
            }
            // anything else isn't drawable
            _ => {}
        }

        next_frame().await
    }
}
