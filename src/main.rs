mod core;
mod ui;

use crate::core::position::Position;
use crate::ui::utils::{
    draw_block_placement_preview, draw_game_stats, draw_game_tile_preview,
    get_window_conf,
};
use core::game::Game;
use core::player::{Player, StartingPosition};
use core::{block, grid};
use macroquad::prelude::*;
use ui::utils::{
    MouseState, draw_block, draw_game_grid_base, draw_player,
    get_mouse_pointer_state, pre_computed_grid_lines,
};

fn window_conf() -> Conf {
    get_window_conf(String::from("quoridor-rust"))
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut blocks: Vec<block::Block> = Vec::with_capacity(20);
    let pre_computed_grid_lines = pre_computed_grid_lines();

    let grid = grid::Grid::new();
    let p1 = Player::new(String::from("p1"), StartingPosition::Top);
    let p2 = Player::new(String::from("p2"), StartingPosition::Bottom);

    let mut game = Game::new(grid, p1, p2);

    loop {
        clear_background(GRAY);

        // draws borders, grid, padding and
        draw_game_grid_base(&pre_computed_grid_lines);

        draw_game_stats();

        // draw blocks that have already been placed
        for b in &blocks {
            draw_block(b.clone(), BLACK);
        }

        // draw players
        draw_player(game.p1.clone());
        draw_player(game.p2.clone());

        let res = get_mouse_pointer_state();
        match &res {
            // hovering above a block position
            MouseState::BlockPosition(block) => {
                // hovering above a block position
                if game.can_add_block(block.clone()) {
                    draw_block_placement_preview(block.clone());
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    if game.add_block(block.clone()) {
                        blocks.push(block.clone());
                    }
                }
            }
            MouseState::RowCol(position) => {
                if !game.is_position_occupied_by_player(&Position {
                    x: position.y,
                    y: position.x,
                }) {
                    draw_game_tile_preview(position);
                }
            }
            // anything else isn't drawable
            _ => {}
        }

        next_frame().await
    }
}
