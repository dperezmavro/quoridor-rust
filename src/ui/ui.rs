use crate::core::block::Block;
use macroquad::prelude::*;

use crate::core::orientation::Orientation;
use crate::core::placement::Placement;
use crate::core::player::Player;
use crate::core::position::Position;

pub const SCREEN_WIDTH: u16 = 800;
pub const TILE_SIZE: f32 = GRID_LINE_WIDTH * 5.0;
const NUM_TILES: usize = 9;
pub const GRID_LINE_WIDTH: f32 = 12.0;
const NUM_GRID_LINES_PER_DIM: usize = NUM_TILES - 1;

const BORDER_WIDTH: f32 = 2.0;

pub const PADDING: f32 = (SCREEN_WIDTH as f32 - TOTAL_LEN) / 2.0;

const TOTAL_LEN: f32 = (TILE_SIZE * NUM_TILES as f32) as f32
    + (NUM_GRID_LINES_PER_DIM as f32 * GRID_LINE_WIDTH);

// Used for determining if the mouse is in the board area
const PADDING_TO_WIDTH_RATIO: f32 = (2.0 * PADDING) / SCREEN_WIDTH as f32;
const PLAYER_RADIUS: f32 = TILE_SIZE / 2.0;
const PLAYER_PADDING: f32 = 3.0;
const PLAYER_FINAL_SIZE: f32 = PLAYER_RADIUS - PLAYER_PADDING;

const LIGHT_GREEN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
    a: 0.4,
};

#[derive(Debug)]
pub enum MouseState {
    RowCol(Position),
    // RowCol { row: u16, col: u16 },
    BlockPosition(Block),
    InBorderCrossSection,
    NotInsideBoard,
}

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
                    h: TOTAL_LEN,
                },
                Rect {
                    x: PADDING,
                    y: PADDING + base_offset,
                    w: TOTAL_LEN,
                    h: GRID_LINE_WIDTH,
                },
            ];
        })
        .flatten()
        .collect();

    grid_lines
}

pub fn draw_base_board(vec: &Vec<Rect>) {
    draw_backdrop();
    draw_bounds();
    draw_grid_precomputed(vec);
}

fn draw_grid_precomputed(vec: &Vec<Rect>) {
    for v in vec {
        draw_rectangle(v.x, v.y, v.w, v.h, WHITE);
    }
}

fn draw_backdrop() {
    draw_rectangle(PADDING, PADDING, TOTAL_LEN, TOTAL_LEN, RED);
}

fn draw_bounds() {
    // left
    draw_rectangle(
        PADDING - BORDER_WIDTH,
        PADDING - BORDER_WIDTH,
        BORDER_WIDTH,
        TOTAL_LEN + BORDER_WIDTH,
        LIGHTGRAY,
    );

    // right
    draw_rectangle(
        PADDING + TOTAL_LEN,
        PADDING,
        BORDER_WIDTH,
        TOTAL_LEN + BORDER_WIDTH,
        LIGHTGRAY,
    );

    // bottom
    draw_rectangle(
        PADDING - BORDER_WIDTH,
        PADDING + TOTAL_LEN,
        TOTAL_LEN + BORDER_WIDTH,
        BORDER_WIDTH,
        LIGHTGRAY,
    );

    // top
    draw_rectangle(
        PADDING - BORDER_WIDTH,
        PADDING - BORDER_WIDTH,
        TOTAL_LEN + BORDER_WIDTH + BORDER_WIDTH,
        BORDER_WIDTH,
        LIGHTGRAY,
    );
}

fn is_mouse_inside_board() -> bool {
    let pos = mouse_position_local();

    pos.x >= -1.0 + PADDING_TO_WIDTH_RATIO
        && pos.x <= 1.0 - PADDING_TO_WIDTH_RATIO
        && pos.y >= -1.0 + PADDING_TO_WIDTH_RATIO
        && pos.y <= 1.0 - PADDING_TO_WIDTH_RATIO
}
pub fn draw_block_placement_preview(block: Block) {
    draw_block(block, LIGHT_GREEN);
}

pub fn get_mouse_pointer_state() -> MouseState {
    if !is_mouse_inside_board() {
        return MouseState::NotInsideBoard;
    }

    let mouse_position = mouse_position();

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

// pub fn draw_block(orientation: &Orientation, tile: u16, lane: u16, c: Color) {
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
