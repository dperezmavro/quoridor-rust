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

/// Number of grid lines - dynamic to the grid
const NUM_GRID_LINES_PER_DIM: usize = NUM_TILES - 1;

/// Total length of the game grid
const GAME_GRID_LEN: f32 = (TILE_SIZE * NUM_TILES as f32) as f32
    + (NUM_GRID_LINES_PER_DIM as f32 * GRID_LINE_WIDTH);

/// Width of box showing player information
const PLAYER_BOX_WIDTH: f32 = 150.0;

/// Total length of all the elements
const TOTAL_LEN: f32 = GAME_GRID_LEN + PLAYER_BOX_WIDTH + (3.0 * PADDING);

/// Screen width, in pixels.
pub const SCREEN_WIDTH: u16 = TOTAL_LEN as u16;

/// Screen height, in pixels
pub const SCREEN_HEIGHT: u16 = PADDING as u16 * 2 + GAME_GRID_LEN as u16;

/// How big to draw the players
const PLAYER_RADIUS: f32 = TILE_SIZE / 2.0;

/// Spacing between player and square edge
const PLAYER_PADDING: f32 = 3.0;

/// Player's final size
const PLAYER_FINAL_SIZE: f32 = PLAYER_RADIUS - PLAYER_PADDING;

/// Constant for light green
const LIGHT_GREEN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
    a: 0.4,
};

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
        window_height: (2 * PADDING as i32) + GAME_GRID_LEN as i32,
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
pub fn draw_base_board(vec: &Vec<Rect>) {
    draw_bounds();
    draw_backdrop();
    draw_grid_precomputed(vec);
    draw_player_stats();
}

fn draw_player_stats() {
    // draw_rectangle();
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
        PADDING,
        PADDING,
        GAME_GRID_LEN,
        GAME_GRID_LEN,
        Color::new(0.90, 0.16, 0.22, 0.70), // light red
    );
}

/// Draws the borders around the game grid
fn draw_bounds() {
    draw_rectangle(
        PADDING - BORDER_WIDTH,
        PADDING - BORDER_WIDTH,
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
    mp.0 >= PADDING
        && mp.0 <= PADDING + GAME_GRID_LEN
        && mp.1 >= PADDING
        && mp.1 <= PADDING + GAME_GRID_LEN
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
        let tile_base =
            PADDING + (y as f32 * TILE_SIZE) + (y as f32 * GRID_LINE_WIDTH);

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
    let dim_1: f32 = PADDING
        + (b.placement.tile as f32 * TILE_SIZE)
        + (b.placement.tile as f32 * GRID_LINE_WIDTH);
    let dim_2: f32 = PADDING
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
        PADDING + ((x as f32) * (TILE_SIZE + GRID_LINE_WIDTH)) + PLAYER_RADIUS,
        PADDING + ((y as f32) * (TILE_SIZE + GRID_LINE_WIDTH)) + PLAYER_RADIUS,
        PLAYER_FINAL_SIZE,
        p.get_color(),
    );
}

/// Draws a preview of the player-tile that the mouse is currently hovering over.
pub fn draw_game_tile_preview(p: &Position) {
    draw_rectangle(
        PADDING + ((p.y as f32) * (TILE_SIZE + GRID_LINE_WIDTH)),
        PADDING + ((p.x as f32) * (TILE_SIZE + GRID_LINE_WIDTH)),
        TILE_SIZE,
        TILE_SIZE,
        Color::new(0.78, 0.48, 1.00, 0.80),
    )
}

#[cfg(test)]
mod tests {
    use crate::ui::utils::{GAME_GRID_LEN, PADDING, is_mouse_inside_board};

    #[test]
    fn test_is_mouse_in_grid_corners() {
        let v = (PADDING, PADDING);
        assert!(is_mouse_inside_board(v));

        let v = (PADDING, PADDING + GAME_GRID_LEN);
        assert!(is_mouse_inside_board(v));

        let v = (PADDING + GAME_GRID_LEN, PADDING);
        assert!(is_mouse_inside_board(v));

        let v = (PADDING + GAME_GRID_LEN, PADDING + GAME_GRID_LEN);
        assert!(is_mouse_inside_board(v));
    }

    #[test]
    fn test_is_mouse_outside_grid() {
        let v = (PADDING, PADDING - 1f32);
        assert!(!is_mouse_inside_board(v));

        let v = (PADDING - 1f32, PADDING + GAME_GRID_LEN);
        assert!(!is_mouse_inside_board(v));

        let v = (PADDING + GAME_GRID_LEN + 1f32, PADDING);
        assert!(!is_mouse_inside_board(v));

        let v = (PADDING + GAME_GRID_LEN, PADDING + GAME_GRID_LEN + 1f32);
        assert!(!is_mouse_inside_board(v));
    }
}
