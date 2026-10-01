use macroquad::prelude::*;

use crate::core::block::Block;
use crate::core::grid::NUM_TILES;
use crate::core::orientation::Orientation;
use crate::core::placement::Placement;
use crate::core::player::{Player, StartingPosition};
use crate::core::position::Position;

/// Width of grid line - this is re-used by the blocks
pub const GRID_LINE_WIDTH: f32 = 12.0;

/// Size of border around the playable board
const BORDER_WIDTH: f32 = 2.0;

/// Padding from the window's edges, in pixels.
pub const PADDING: f32 = 50.0;

/// Width of box showing player information
const GAME_STATS_BOX_WIDTH: f32 = 150.0;

/// Spacing between player and square edge
const PLAYER_PADDING: f32 = 3.0;

/// Size of each tile, in pixes.
pub const TILE_SIZE: f32 = GRID_LINE_WIDTH * 5.0;

/// How big to draw the players
const PLAYER_RADIUS: f32 = TILE_SIZE / 2.0;

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

/// Constant for light red
const LIGHT_RED: Color = Color::new(0.90, 0.16, 0.22, 0.70);

const LIGHT_BLUE: Color = Color { a: 0.7, ..BLUE };

const LIGHT_GREEN: Color = Color { a: 0.7, ..LIME };

const LIGHT_YELLOW: Color = Color { a: 0.6, ..YELLOW };

/// Padding for the box showing game stats
const GAME_STATS_BOX_PADDING: f32 = 5.0;

/// Font size for H1
const FONT_SIZE_H1: u16 = 60;

/// Font size for H3
const FONT_SIZE_H3: u16 = FONT_SIZE_H1 - 30u16;

/// Convenience offset for where to start drawing the text in the stats box
const STATS_X_OFFSET: f32 = GAME_STATS_BOX_START + (GAME_STATS_BOX_WIDTH / 2.0);

/// First player's colour mapping
const FIRST_PLAYER_ID: u8 = 0;

/// Describes what the mouse is hovering over.
#[derive(Debug, Clone, PartialEq)]
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
        .flat_map(|x| {
            let base_offset: f32 =
                (x + 1) as f32 * TILE_SIZE + (x as f32 * GRID_LINE_WIDTH);
            vec![
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
            ]
        })
        .collect();

    grid_lines
}

/// Draws the base elements - backdrop, bounds and grid
pub fn draw_game_grid_base(vec: &Vec<Rect>) {
    draw_bounds();
    draw_backdrop();
    draw_grid_precomputed(vec);
}

/// Used for drawing player information
pub fn draw_game_stats(f: Option<&Font>, block_counts: (usize, usize)) {
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

    let mut next_line_offset = PADDING + (2.0 * GAME_STATS_BOX_PADDING);

    // H1
    let h1_texts = [String::from("game"), String::from("stats")];
    h1_texts.iter().for_each(|h| {
        let h1_string = h;
        let h1_text_centre = get_text_center(
            h1_string.clone(),
            f,
            h1_params.font_size,
            1.0,
            0.0,
        );

        next_line_offset +=
            (h1_text_centre.y.abs() + GAME_STATS_BOX_PADDING) * 2.0;

        // GAME
        draw_text_ex(
            h1_string.clone(),
            STATS_X_OFFSET - h1_text_centre.x,
            next_line_offset,
            h1_params.clone(),
        );
    });

    next_line_offset += GAME_STATS_BOX_PADDING * 12.0;

    let h3_params = TextParams {
        font_size: FONT_SIZE_H3,
        ..h1_params
    };

    // H3
    let h3_texts = [
        String::from("player 1"),
        format!("blocks: {}", block_counts.0),
        String::from("player 2"),
        format!("blocks: {}", block_counts.1),
    ];

    h3_texts.iter().enumerate().for_each(|(i, h)| {
        let h3_string = h.to_string();
        if i == 2 {
            // text for player 2
            next_line_offset += 4.0 * GAME_STATS_BOX_PADDING;
        }
        let h3_string_text_centre = get_text_center(
            h3_string.clone(),
            f,
            h3_params.font_size,
            1.0,
            0.0,
        );

        draw_text_ex(
            h3_string.clone(),
            STATS_X_OFFSET - h3_string_text_centre.x,
            next_line_offset,
            h3_params.clone(),
        );

        next_line_offset +=
            2.0 * h3_string_text_centre.y.abs() + GAME_STATS_BOX_PADDING;
    });
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

fn draw_tile_at(p: Position) {
    draw_rectangle(
        PADDING + (p.x as f32 * (GRID_LINE_WIDTH + TILE_SIZE)),
        PADDING + (p.y as f32 * (GRID_LINE_WIDTH + TILE_SIZE)),
        TILE_SIZE,
        TILE_SIZE,
        WHITE,
    )
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
pub fn get_current_player_color(player_id: u8) -> Color {
    let mut c = LIGHT_BLUE;
    if player_id == FIRST_PLAYER_ID {
        c = LIGHT_YELLOW;
    }
    c
}

/// Draws a preview of where a block could go
pub fn draw_placed_block(block: Block) {
    draw_block(block, LIGHT_GREEN);
}

/// Draws a preview of where a block could go
pub fn draw_block_placement_preview(block: Block, player_id: u8) {
    draw_block(block, get_current_player_color(player_id));
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
pub fn get_mouse_pointer_state(mouse_position: (f32, f32)) -> MouseState {
    if !is_mouse_inside_board(mouse_position) {
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
            x: col as usize,
            y: row as usize,
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
pub fn draw_player(p: &Player, highlight: bool) {
    let x = p.position.x;
    let y = p.position.y;
    let color = match p.starting_position {
        StartingPosition::Top => {
            if highlight {
                LIGHT_YELLOW
            } else {
                YELLOW
            }
        }
        StartingPosition::Bottom => {
            if highlight {
                LIGHT_BLUE
            } else {
                BLUE
            }
        }
    };

    if highlight {
        draw_tile_at(p.position);
    }

    draw_circle(
        GAME_GRID_BOX_START
            + ((x as f32) * (TILE_SIZE + GRID_LINE_WIDTH))
            + PLAYER_RADIUS,
        GAME_GRID_BOX_START
            + ((y as f32) * (TILE_SIZE + GRID_LINE_WIDTH))
            + PLAYER_RADIUS,
        PLAYER_FINAL_SIZE,
        color,
    );
}

/// Highlights player options around the player
pub fn highlight_player_options(player_id: u8, pos: &Vec<Position>) {
    for p in pos {
        draw_rectangle(
            GAME_GRID_BOX_START
                + ((p.x as f32) * (TILE_SIZE + GRID_LINE_WIDTH)),
            GAME_GRID_BOX_START
                + ((p.y as f32) * (TILE_SIZE + GRID_LINE_WIDTH)),
            TILE_SIZE,
            TILE_SIZE,
            get_current_player_color(player_id),
        );
    }
}

/// Draws a preview of the player-tile
/// that the mouse is currently hovering over.
/// if it presents a valid move
pub fn draw_move_preview(p: &Position) {
    draw_rectangle(
        GAME_GRID_BOX_START + ((p.x as f32) * (TILE_SIZE + GRID_LINE_WIDTH)),
        GAME_GRID_BOX_START + ((p.y as f32) * (TILE_SIZE + GRID_LINE_WIDTH)),
        TILE_SIZE,
        TILE_SIZE,
        Color::new(0.78, 0.48, 1.00, 0.80),
    )
}

#[cfg(test)]
mod tests {
    use crate::core::block::Block;
    use crate::core::orientation::Orientation;
    use crate::core::placement::Placement;
    use crate::ui::utils::{
        GAME_GRID_BOX_START, GAME_GRID_LEN, LIGHT_BLUE, LIGHT_YELLOW,
        MouseState, PADDING, TILE_SIZE, get_current_player_color,
        get_mouse_pointer_state, is_mouse_inside_board,
        pre_computed_grid_lines,
    };

    use crate::core::position::Position;

    #[test]
    fn test_correct_num_of_precomputed_lines() {
        let pcl = pre_computed_grid_lines();
        assert_eq!(16, pcl.len());
    }

    #[test]
    fn test_correct_colours() {
        assert_eq!(LIGHT_YELLOW, get_current_player_color(0));
        assert_eq!(LIGHT_BLUE, get_current_player_color(1));
    }

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

    #[test]
    fn test_get_mouse_pointer_state() {
        let mps = get_mouse_pointer_state((0.0, 0.0));
        assert_eq!(mps, MouseState::NotInsideBoard);

        let mps = get_mouse_pointer_state((PADDING + 1.0, PADDING + 1.0));
        assert_eq!(mps, MouseState::RowCol(Position { x: 0, y: 0 }));

        let mps = get_mouse_pointer_state((PADDING, PADDING + TILE_SIZE + 1.0));
        assert_eq!(
            mps,
            MouseState::BlockPosition(Block::new(
                Placement::new(0, 0),
                Orientation::Horizontal
            ))
        );

        let mps = get_mouse_pointer_state((
            PADDING + TILE_SIZE + 1.0,
            PADDING + TILE_SIZE + 1.0,
        ));
        assert_eq!(mps, MouseState::InBorderCrossSection);
    }
}
