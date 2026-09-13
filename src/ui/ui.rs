use macroquad::prelude::*;
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

#[derive(Debug, Copy, Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

// struct BlockPosition{
//     orientation: Orientation,
//     tile: u16,
//     lane: u16
// }
#[derive(Debug)]
pub enum MouseState {
    RowCol {
        row: u16,
        col: u16,
    },
    BlockPosition {
        orientation: Orientation,
        tile: u16,
        lane: u16,
    },
    InBorderCrossSection,
    NotInsideBoard,
}

pub struct Block {
    pub orientation: Orientation,
    pub tile: u16,
    pub lane: u16,
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

pub fn draw_grid_precomputed(vec: &Vec<Rect>) {
    for v in vec {
        draw_rectangle(v.x, v.y, v.w, v.h, WHITE);
    }
}

pub fn draw_backdrop() {
    draw_rectangle(PADDING, PADDING, TOTAL_LEN, TOTAL_LEN, RED);
}

pub fn draw_bounds() {
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
        let mut tile_base =
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
        MouseState::RowCol { row, col }
    } else if in_horizontal_block_pos && in_vertical_block_pos {
        MouseState::InBorderCrossSection
    } else {
        if found_row {
            return MouseState::BlockPosition {
                orientation: Orientation::Vertical,
                tile: row,
                lane,
            };
        }

        MouseState::BlockPosition {
            orientation: Orientation::Horizontal,
            tile: col,
            lane,
        }
    }
}

pub fn draw_block(orientation: &Orientation, tile: &u16, lane: &u16) {
    if *tile >= NUM_TILES as u16 - 1 {
        return;
    }
    let dim_1: f32 =
        PADDING + (*tile as f32 * TILE_SIZE) + (*tile as f32 * GRID_LINE_WIDTH);
    let dim_2: f32 = PADDING
        + ((*lane + 1) as f32 * TILE_SIZE)
        + (*lane as f32 * GRID_LINE_WIDTH);
    let long_side: f32 = (TILE_SIZE * 2.0) + GRID_LINE_WIDTH;

    let (x, y, w, h) = match orientation {
        Orientation::Horizontal => (dim_1, dim_2, long_side, GRID_LINE_WIDTH),
        Orientation::Vertical => (dim_2, dim_1, GRID_LINE_WIDTH, long_side),
    };
    draw_rectangle(x, y, w, h, BLACK);
}
