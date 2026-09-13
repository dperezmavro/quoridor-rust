mod block;
mod game;
mod grid;
mod player;
mod position;
mod tile;

use game::Game;

mod placement;
mod ui;

use ui::ui::{
    Block, GRID_LINE_WIDTH, MouseState, PADDING, SCREEN_WIDTH, TILE_SIZE,
    draw_base_board, draw_block, get_mouse_pointer_state,
    pre_computed_grid_lines,
};

// fn main() {
//     let mut g = Game::new(String::from("d"), String::from("k"));
//     g.start();
//
//     loop {}
// }

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "quoridor-rust".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_WIDTH as i32,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut blocks: Vec<Block> = Vec::with_capacity(20);
    let pre_computed_grid_lines = pre_computed_grid_lines();

    let mut g = Game::new(String::from("d"), String::from("k"));
    // currently noop
    g.start();

    loop {
        clear_background(GRAY);

        draw_base_board(&pre_computed_grid_lines);

        let res = get_mouse_pointer_state();
        match &res {
            MouseState::BlockPosition {
                orientation,
                tile,
                lane,
            } => draw_block(orientation, tile, lane),
            MouseState::RowCol { row, col } => draw_rectangle(
                PADDING
                    + (*col as f32 * TILE_SIZE)
                    + (*col as f32 * GRID_LINE_WIDTH),
                PADDING
                    + (*row as f32 * TILE_SIZE)
                    + (*row as f32 * GRID_LINE_WIDTH),
                TILE_SIZE,
                TILE_SIZE,
                PURPLE,
            ),
            // anything else isn't drawable
            _ => {}
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            match &res {
                MouseState::BlockPosition {
                    orientation,
                    tile,
                    lane,
                } => blocks.push(Block {
                    orientation: *orientation,
                    tile: *tile,
                    lane: *lane,
                }),
                // only care about the block kind
                _ => {}
            }
        }

        for b in &blocks {
            draw_block(&b.orientation, &b.tile, &b.lane);
        }

        next_frame().await
    }
}
