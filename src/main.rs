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
    GRID_LINE_WIDTH, MouseState, PADDING, SCREEN_WIDTH, TILE_SIZE,
    draw_base_board, draw_block, draw_player, get_mouse_pointer_state,
    pre_computed_grid_lines,
};

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
    let mut blocks: Vec<block::Block> = Vec::with_capacity(20);
    let pre_computed_grid_lines = pre_computed_grid_lines();

    let grid = grid::Grid::new();
    let mut game = Game::new(String::from("d"), String::from("k"), grid);

    loop {
        clear_background(GRAY);

        // draws borders, grid, padding and
        draw_base_board(&pre_computed_grid_lines);

        for b in &blocks {
            draw_block(
                &b.orientation,
                b.placement.tile as u16,
                b.placement.lane as u16,
                BLACK,
            );
        }

        //
        draw_player(game.p1.position.x as u16, game.p1.position.y as u16, BLUE);
        draw_player(
            game.p2.position.x as u16,
            game.p2.position.y as u16,
            YELLOW,
        );

        let res = get_mouse_pointer_state();
        match &res {
            // hovering above a block position
            MouseState::BlockPosition {
                orientation,
                tile,
                lane,
            } => {
                // hovering above a block position
                if game.can_add_block(block::Block {
                    orientation: *orientation,
                    placement: placement::Placement {
                        tile: *tile as usize,
                        lane: *lane as usize,
                    },
                }) {
                    draw_block(orientation, *tile, *lane, GREEN);
                }
            }
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
                } => {
                    let block = block::Block {
                        orientation: orientation.clone(),
                        placement: placement::Placement {
                            tile: *tile as usize,
                            lane: *lane as usize,
                        },
                    };
                    if game.add_block(block) {
                        blocks.push(block);
                    }
                }
                // only care about the block kind
                _ => {}
            }
        }

        next_frame().await
    }
}
