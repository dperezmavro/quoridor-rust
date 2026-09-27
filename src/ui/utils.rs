use macroquad::prelude::*;

use crate::core::block::Block;
use crate::core::grid::NUM_TILES;
use crate::core::orientation::Orientation;
use crate::core::placement::Placement;
use crate::core::player::Player;
use crate::core::position::Position;

/// Size of each tile, in pixes.
pub const TILE_SIZE: f32 = GRID_LINE_WIDTH * 5.0;

/// Width of grid line - this is re-used by the blocks
pub const GRID_LINE_WIDTH: f32 = 12.0;

/// Size of border around the playable board
const BORDER_WIDTH: f32 = 2.0;

/// Padding from the window's edges, in pixels.
pub const PADDING: f32 = 50.0;

/// Width of box showing player information
const GAME_STATS_BOX_WIDTH: f32 = 150.0;

/// How big to draw the players
const PLAYER_RADIUS: f32 = TILE_SIZE / 2.0;

/// Spacing between player and square edge
const PLAYER_PADDING: f32 = 3.0;

/// Where the game grid starts
pub const GAME_GRID_BOX_START: f32 = PADDING;

/// Number of grid lines - dynamic to the grid
const NUM_GRID_LINES_PER_DIM: usize = NUM_TILES - 1;

/// Total length of the game grid
const GAME_GRID_LEN: f32 = (TILE_SIZE * NUM_TILES as f32)
    + (NUM_GRID_LINES_PER_DIM as f32 * GRID_LINE_WIDTH);

/// Total length of all the elements, including padding
const TOTAL_LEN: f32 = GAME_GRID_LEN + GAME_STATS_BOX_WIDTH + (3.0 * PADDING);

/// Screen width, in pixels.
pub const SCREEN_WIDTH: u16 = TOTAL_LEN as u16;

/// Screen height, in pixels
pub const SCREEN_HEIGHT: u16 = PADDING as u16 * 2 + GAME_GRID_LEN as u16;

/// Player's final size
const PLAYER_FINAL_SIZE: f32 = PLAYER_RADIUS - PLAYER_PADDING;

/// Where the game display box starts
const GAME_STATS_BOX_START: f32 = (2.0 * PADDING) + GAME_GRID_LEN;

/// Constant for light green
const LIGHT_GREEN: Color = Color::new(0.0, 1.0, 0.0, 0.4);

/// Constant for light red
const LIGHT_RED: Color = Color::new(0.90, 0.16, 0.22, 0.70);

/// Describes what the mouse is hovering over.
#[derive(Debug, Clone)]
pub enum MouseState {
    /// Used when mouse is above player tile.
    RowCol(Position),

    /// Used when mouse is above a block position.
    BlockPosition(Block),

    /// Mouse in border cross-section - dead region.
    InBorderCrossSection,

    /// Outside playable grid - nothing to do.
    NotInsideBoard,
}

/// Returns the default window config
pub fn get_window_conf(title: String) -> Conf {
    Conf {
        window_title: title,
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

/// Pre-computes the grid's lines where blocks will be placed
pub fn pre_computed_grid_lines() -> Vec<Rect> {
    let grid_lines: Vec<Rect> = (0..NUM_GRID_LINES_PER_DIM)
        .map(|x| {
            let base_offset: f32 =
                (x + 1) as f32 * TILE_SIZE + (x as f32 * GRID_LINE_WIDTH);
            return vec![
                Rect {
                    x: PADDING + base_offset,
                    y: PADDING,
                    w: GRID_LINE_WIDTH,
                    h: GAME_GRID_LEN,
                },
                Rect {
                    x: PADDING,
                    y: PADDING + base_offset,
                    w: GAME_GRID_LEN,
                    h: GRID_LINE_WIDTH,
                },
            ];
        })
        .flatten()
        .collect();

    grid_lines
}

/// Draws the base elements - backdrop, bounds and grid
pub fn draw_game_grid_base(vec: &Vec<Rect>) {
    draw_bounds();
    draw_backdrop();
    draw_grid_precomputed(vec);
}

const GAME_STATS_BOX_PADDING: f32 = 5.0;

const FONT_SIZE_H1: u16 = 60;

const FONE_SIZE_H3: u16 = FONT_SIZE_H1 - 30u16;

const STATS_X_OFFSET: f32 = GAME_STATS_BOX_START + (GAME_STATS_BOX_WIDTH / 2.0);

pub fn draw_game_stats(text: String, f: Option<&Font>) {
    draw_rectangle(
        GAME_STATS_BOX_START,
        PADDING,
        GAME_STATS_BOX_WIDTH,
        GAME_GRID_LEN,
        WHITE,
    );

    let h1_params = TextParams {
        font: f,
        font_size: FONT_SIZE_H1,
        color: BLACK,
        ..Default::default()
    };

    let game_string = String::from("game");
    let game_text_centre =
        get_text_center(game_string.clone(), f, h1_params.font_size, 1.0, 0.0);

    let mut next_line_offset = PADDING
        + (3.0 * GAME_STATS_BOX_PADDING)
        + (game_text_centre.y.abs() * 2.0);

    // GAME
    draw_text_ex(
        game_string.clone(),
        STATS_X_OFFSET - game_text_centre.x,
        next_line_offset,
        h1_params.clone(),
    );

    let stats_string = String::from("stats");
    let stats_text_centre =
        get_text_center(stats_string.clone(), f, h1_params.font_size, 1.0, 0.0);
    next_line_offset +=
        2.0 * (game_text_centre.y.abs() + GAME_STATS_BOX_PADDING);
    // STATS
    draw_text_ex(
        stats_string.clone(),
        STATS_X_OFFSET - stats_text_centre.x,
        next_line_offset,
        h1_params.clone(),
    );
    let h3_params = TextParams {
        font: f,
        font_size: FONE_SIZE_H3,
        color: BLACK, // Classic digital clock green (or RED/YELLOW)
        ..Default::default()
    };

    let player_1_string = String::from("player 1");
    let player_1_text_centre = get_text_center(
        player_1_string.clone(),
        f,
        h3_params.font_size,
        1.0,
        0.0,
    );
    next_line_offset +=
        (2.0 * game_text_centre.y.abs()) + GAME_STATS_BOX_PADDING;
    // PLAYER 1
    draw_text_ex(
        player_1_string.clone(),
        STATS_X_OFFSET - player_1_text_centre.x,
        next_line_offset,
        h3_params.clone(),
    );

    let player_1_blocks_string = String::from("blocks: 10");
    let player_1_blocks_string_centre = get_text_center(
        player_1_blocks_string.clone(),
        f,
        h3_params.font_size,
        1.0,
        0.0,
    );
    next_line_offset +=
        2.0 * (player_1_blocks_string_centre.y.abs() + GAME_STATS_BOX_PADDING);
    // GAME
    draw_text_ex(
        player_1_blocks_string.clone(),
        STATS_X_OFFSET - player_1_blocks_string_centre.x,
        next_line_offset,
        h3_params.clone(),
    );

    let player_2_string = String::from("player 2");
    let player_2_text_centre = get_text_center(
        player_2_string.clone(),
        f,
        h3_params.font_size,
        1.0,
        0.0,
    );
    next_line_offset += (2.0 * player_1_blocks_string_centre.y.abs())
        + (4.0 * GAME_STATS_BOX_PADDING);
    // PLAYER 2
    draw_text_ex(
        player_2_string.clone(),
        STATS_X_OFFSET - player_2_text_centre.x,
        next_line_offset,
        h3_params.clone(),
    );

    let player_2_blocks_string = String::from("blocks: 10");
    let player_2_blocks_string_centre = get_text_center(
        player_2_blocks_string.clone(),
        f,
        h3_params.font_size,
        1.0,
        0.0,
    );
    next_line_offset +=
        2.0 * (player_2_blocks_string_centre.y.abs() + GAME_STATS_BOX_PADDING);
    // GAME
    draw_text_ex(
        player_2_blocks_string.clone(),
        STATS_X_OFFSET - player_2_blocks_string_centre.x,
        next_line_offset,
        h3_params.clone(),
    );
}

/// Draws the grid
fn draw_grid_precomputed(vec: &Vec<Rect>) {
    for v in vec {
        draw_rectangle(v.x, v.y, v.w, v.h, WHITE);
    }
}
/// Draws the backdrop
fn draw_backdrop() {
    draw_rectangle(
        GAME_GRID_BOX_START,
        GAME_GRID_BOX_START,
        GAME_GRID_LEN,
        GAME_GRID_LEN,
        LIGHT_RED,
    );
}

/// Draws the borders around the game grid
fn draw_bounds() {
    draw_rectangle(
        GAME_GRID_BOX_START - BORDER_WIDTH,
        GAME_GRID_BOX_START - BORDER_WIDTH,
        GAME_GRID_LEN + BORDER_WIDTH + BORDER_WIDTH,
        GAME_GRID_LEN + BORDER_WIDTH + BORDER_WIDTH,
        LIGHTGRAY,
    );
}

/// Draws a preview of where a block could go
pub fn draw_block_placement_preview(block: Block) {
    draw_block(block, LIGHT_GREEN);
}

/// Tests whether the mouse is inside the game grid
fn is_mouse_inside_board(mp: (f32, f32)) -> bool {
    mp.0 >= GAME_GRID_BOX_START
        && mp.0 <= GAME_GRID_BOX_START + GAME_GRID_LEN
        && mp.1 >= GAME_GRID_BOX_START
        && mp.1 <= GAME_GRID_BOX_START + GAME_GRID_LEN
}

/// Figure out what state the mouse pointer is in.
///
/// This determines what can be done with this event (place block, highlight player tile etc).
pub fn get_mouse_pointer_state() -> MouseState {
    let mouse_position = mouse_position();
    if !is_mouse_inside_board(mouse_position.clone()) {
        return MouseState::NotInsideBoard;
    }

    let mut row: u16 = 0;
    let mut found_row: bool = false;
    let mut in_horizontal_block_pos = false;
    let mut lane: u16 = 0;

    let mut col: u16 = 0;
    let mut found_col: bool = false;
    let mut in_vertical_block_pos = false;

    for y in 0..NUM_TILES {
        let tile_base = GAME_GRID_BOX_START
            + (y as f32 * TILE_SIZE)
            + (y as f32 * GRID_LINE_WIDTH);

        if mouse_position.0 >= tile_base {
            if mouse_position.0 <= tile_base + TILE_SIZE {
                col = y as u16;
                found_col = true;
            } else if mouse_position.0
                <= tile_base + TILE_SIZE + GRID_LINE_WIDTH
            {
                in_horizontal_block_pos = true;
                lane = y as u16;
            }
        }

        // detect if it is in square
        if mouse_position.1 >= tile_base {
            if mouse_position.1 <= tile_base + TILE_SIZE {
                row = y as u16;
                found_row = true;
            } else if mouse_position.1
                <= tile_base + TILE_SIZE + GRID_LINE_WIDTH
            {
                // detect if it is in block
                in_vertical_block_pos = true;
                lane = y as u16;
            }
        }
    }

    if found_row && found_col {
        MouseState::RowCol(Position {
            x: row as usize,
            y: col as usize,
        })
    } else if in_horizontal_block_pos && in_vertical_block_pos {
        MouseState::InBorderCrossSection
    } else {
        if found_row {
            return MouseState::BlockPosition(Block {
                orientation: Orientation::Vertical,
                placement: Placement {
                    tile: row as usize,
                    lane: lane as usize,
                },
            });
        }

        MouseState::BlockPosition(Block {
            orientation: Orientation::Horizontal,
            placement: Placement {
                tile: col as usize,
                lane: lane as usize,
            },
        })
    }
}

/// Draws a block - wrapper method for the macroquad method
pub fn draw_block(b: Block, c: Color) {
    if b.placement.tile as u16 >= NUM_TILES as u16 - 1 {
        return;
    }
    let dim_1: f32 = GAME_GRID_BOX_START
        + (b.placement.tile as f32 * TILE_SIZE)
        + (b.placement.tile as f32 * GRID_LINE_WIDTH);
    let dim_2: f32 = GAME_GRID_BOX_START
        + ((b.placement.lane + 1) as f32 * TILE_SIZE)
        + (b.placement.lane as f32 * GRID_LINE_WIDTH);
    let long_side: f32 = (TILE_SIZE * 2.0) + GRID_LINE_WIDTH;

    let (x, y, w, h) = match b.orientation {
        Orientation::Horizontal => (dim_1, dim_2, long_side, GRID_LINE_WIDTH),
        Orientation::Vertical => (dim_2, dim_1, GRID_LINE_WIDTH, long_side),
    };
    draw_rectangle(x, y, w, h, c);
}

/// Draws a circle representing a player on the grid
pub fn draw_player(p: Player) {
    let x = p.position.x;
    let y = p.position.y;
    draw_circle(
        GAME_GRID_BOX_START
            + ((x as f32) * (TILE_SIZE + GRID_LINE_WIDTH))
            + PLAYER_RADIUS,
        GAME_GRID_BOX_START
            + ((y as f32) * (TILE_SIZE + GRID_LINE_WIDTH))
            + PLAYER_RADIUS,
        PLAYER_FINAL_SIZE,
        p.get_color(),
    );
}

/// Draws a preview of the player-tile that the mouse is currently hovering over.
pub fn draw_game_tile_preview(p: &Position) {
    draw_rectangle(
        GAME_GRID_BOX_START + ((p.y as f32) * (TILE_SIZE + GRID_LINE_WIDTH)),
        GAME_GRID_BOX_START + ((p.x as f32) * (TILE_SIZE + GRID_LINE_WIDTH)),
        TILE_SIZE,
        TILE_SIZE,
        Color::new(0.78, 0.48, 1.00, 0.80),
    )
}

#[cfg(test)]
mod tests {
    use crate::ui::utils::{
        GAME_GRID_BOX_START, GAME_GRID_LEN, is_mouse_inside_board,
    };

    #[test]
    fn test_is_mouse_in_grid_corners() {
        let v = (GAME_GRID_BOX_START, GAME_GRID_BOX_START);
        assert!(is_mouse_inside_board(v));

        let v = (GAME_GRID_BOX_START, GAME_GRID_BOX_START + GAME_GRID_LEN);
        assert!(is_mouse_inside_board(v));

        let v = (GAME_GRID_BOX_START + GAME_GRID_LEN, GAME_GRID_BOX_START);
        assert!(is_mouse_inside_board(v));

        let v = (
            GAME_GRID_BOX_START + GAME_GRID_LEN,
            GAME_GRID_BOX_START + GAME_GRID_LEN,
        );
        assert!(is_mouse_inside_board(v));
    }

    #[test]
    fn test_is_mouse_outside_grid() {
        let v = (GAME_GRID_BOX_START, GAME_GRID_BOX_START - 1f32);
        assert!(!is_mouse_inside_board(v));

        let v = (
            GAME_GRID_BOX_START - 1f32,
            GAME_GRID_BOX_START + GAME_GRID_LEN,
        );
        assert!(!is_mouse_inside_board(v));

        let v = (
            GAME_GRID_BOX_START + GAME_GRID_LEN + 1f32,
            GAME_GRID_BOX_START,
        );
        assert!(!is_mouse_inside_board(v));

        let v = (
            GAME_GRID_BOX_START + GAME_GRID_LEN,
            GAME_GRID_BOX_START + GAME_GRID_LEN + 1f32,
        );
        assert!(!is_mouse_inside_board(v));
    }
}
