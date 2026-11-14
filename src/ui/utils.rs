use std::cell::LazyCell;

use macroquad::prelude::*;

use crate::core::grid::NUM_TILES;
use crate::core::orientation::Orientation;
use crate::core::placement::Placement;
use crate::core::player::{Player, StartingPosition};
use crate::core::position::Position;
use crate::core::wall::Wall;
use crate::ui::text_panel::{TextHeading, TextPanel};
use crate::ui::ui_handler::UIHandler;

/// Width of grid line - this is re-used by the walls
pub const WALL_WIDTH: f32 = 12.0;

/// Size of border around the playable board
const BORDER_WIDTH: f32 = 2.0;

/// Padding from the window's edges, in pixels.
pub const PADDING: f32 = 50.0;

/// Width of box showing player information
pub const GAME_STATS_BOX_WIDTH: f32 = 150.0;

/// Spacing between player and square edge
const PLAYER_PADDING: f32 = 3.0;

/// Size of each tile, in pixes.
pub const TILE_SIZE: f32 = WALL_WIDTH * 5.0;

/// How big to draw the players
const PLAYER_RADIUS: f32 = TILE_SIZE / 2.0;

/// Where the game grid starts
pub const GAME_GRID_BOX_START: f32 = PADDING;

/// Number of grid lines - dynamic to the grid
const NUM_GRID_LINES_PER_DIM: usize = NUM_TILES - 1;

/// Total length of the game grid
const GAME_GRID_LEN: f32 = (TILE_SIZE * NUM_TILES as f32)
    + (NUM_GRID_LINES_PER_DIM as f32 * WALL_WIDTH);

/// Total length of all the elements, including padding
const TOTAL_LEN: f32 = GAME_GRID_LEN + GAME_STATS_BOX_WIDTH + (3.0 * PADDING);

/// Screen width, in pixels.
pub const SCREEN_WIDTH: u16 = TOTAL_LEN as u16;

/// Screen height, in pixels
pub const SCREEN_HEIGHT: u16 = PADDING as u16 * 2 + GAME_GRID_LEN as u16;

/// Player's final size
const PLAYER_FINAL_SIZE: f32 = PLAYER_RADIUS - PLAYER_PADDING;

/// Where the game display box starts
pub const GAME_STATS_BOX_START: f32 = (2.0 * PADDING) + GAME_GRID_LEN;

/// Constant for light red
const LIGHT_RED: Color = Color::new(0.90, 0.16, 0.22, 0.70);

const LIGHT_BLUE: Color = Color { a: 0.7, ..BLUE };

const LIGHT_GREEN: Color = Color { a: 0.7, ..LIME };

const LIGHT_YELLOW: Color = Color { a: 0.6, ..YELLOW };

/// First player's colour mapping
const FIRST_PLAYER_ID: u8 = 0;

/// Describes what the mouse is hovering over.
#[derive(Debug, Clone, PartialEq)]
pub enum MouseState {
    /// Used when mouse is above player tile.
    RowCol(Position),

    /// Used when mouse is above a wall position.
    WallPosition(Wall),

    /// Mouse in border cross-section - dead region.
    InBorderCrossSection,

    /// Outside playable grid - nothing to do.
    NotInsideBoard,
}

/// Returns the default window config
pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from("quoridor-rs"),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

/// Pre-computes the grid's lines where walls will be placed
#[allow(clippy::declare_interior_mutable_const)]
const GRID_LINES_ARR: LazyCell<[Rect; 2 * NUM_GRID_LINES_PER_DIM]> =
    LazyCell::new(|| {
        let mut arr: [Rect; 2 * NUM_GRID_LINES_PER_DIM] =
            [Rect::default(); 2 * NUM_GRID_LINES_PER_DIM];
        for x in 0..NUM_GRID_LINES_PER_DIM {
            let base_offset: f32 =
                (x + 1) as f32 * TILE_SIZE + (x as f32 * WALL_WIDTH);
            arr[x * 2] = Rect {
                x: PADDING + base_offset,
                y: PADDING,
                w: WALL_WIDTH,
                h: GAME_GRID_LEN,
            };
            arr[(x * 2) + 1] = Rect {
                x: PADDING,
                y: PADDING + base_offset,
                w: GAME_GRID_LEN,
                h: WALL_WIDTH,
            };
        }

        arr
    });

/// Draws the base elements - backdrop, bounds and grid
pub fn draw_game_grid_base(
    lines: &LazyCell<[Rect; 2 * NUM_GRID_LINES_PER_DIM]>,
) {
    draw_bounds();
    draw_backdrop();
    draw_grid_precomputed(lines);
}

/// Used for drawing player information
pub fn draw_game_stats<T>(
    uih: &T,
    font: Option<&Font>,
    wall_counts: (usize, usize),
    score: (usize, usize),
) where
    T: UIHandler,
{
    draw_rectangle(
        GAME_STATS_BOX_START,
        PADDING,
        GAME_STATS_BOX_WIDTH,
        GAME_GRID_LEN,
        WHITE,
    );

    // let next_line_offset =
    let mut tp = TextPanel::new(PADDING);

    tp.draw_text(uih, font, TextHeading::H1, vec!["game", "stats"]);

    tp.draw_text(
        uih,
        font,
        TextHeading::H3,
        vec![
            "player 1",
            format!("walls: {}", wall_counts.0).as_str(),
            "player 2",
            format!("walls: {}", wall_counts.1).as_str(),
        ],
    );

    tp.draw_text(
        uih,
        font,
        TextHeading::H2,
        vec!["Score", format!("{} - {}", score.0, score.1).as_str()],
    );
}

pub struct Ui {
    font: Font,
    grid_lines: LazyCell<[Rect; 2 * NUM_GRID_LINES_PER_DIM]>,
}

impl UIHandler for Ui {
    async fn new() -> Self {
        let font = load_ttf_font("./assets/digital-7.ttf").await.unwrap();
        Ui {
            font,
            grid_lines: GRID_LINES_ARR,
        }
    }

    fn draw_game_base(
        &self,
        remaining_walls: (usize, usize),
        scores: (usize, usize),
    ) {
        clear_background(GRAY);

        // draws borders, grid, padding and
        draw_game_grid_base(&self.grid_lines);

        draw_game_stats(self, Some(&self.font), remaining_walls, scores);
    }

    fn mouse_position(&self) -> (f32, f32) {
        mouse_position()
    }

    fn is_mouse_button_pressed(&self) -> bool {
        is_mouse_button_pressed(MouseButton::Left)
    }

    async fn next_frame(&self) {
        next_frame().await
    }

    /// Draws a circle representing a player on the grid
    fn draw_player(&self, p: &Player, highlight: bool) {
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

        self.draw_circle(
            GAME_GRID_BOX_START
                + ((x as f32) * (TILE_SIZE + WALL_WIDTH))
                + PLAYER_RADIUS,
            GAME_GRID_BOX_START
                + ((y as f32) * (TILE_SIZE + WALL_WIDTH))
                + PLAYER_RADIUS,
            PLAYER_FINAL_SIZE,
            color,
        );
    }

    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        draw_rectangle(x, y, w, h, color);
    }

    fn draw_circle(&self, x: f32, y: f32, r: f32, color: Color) {
        draw_circle(x, y, r, color);
    }

    fn draw_text_ex(
        &self,
        text: impl AsRef<str>,
        x: f32,
        y: f32,
        params: TextParams,
    ) {
        draw_text_ex(text, x, y, params);
    }
}

/// Draws the grid
fn draw_grid_precomputed(lines: &LazyCell<[Rect; 2 * NUM_GRID_LINES_PER_DIM]>) {
    for v in lines.iter() {
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

/// Used for highlighting tiles
fn draw_tile_at(p: Position) {
    draw_rectangle(
        PADDING + (p.x as f32 * (WALL_WIDTH + TILE_SIZE)),
        PADDING + (p.y as f32 * (WALL_WIDTH + TILE_SIZE)),
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

/// Draws a preview of where a wall could go
pub fn get_current_player_color(player_id: u8) -> Color {
    if player_id == FIRST_PLAYER_ID {
        LIGHT_YELLOW
    } else {
        LIGHT_BLUE
    }
}

/// Draws a preview of where a wall could go
pub fn draw_placed_wall<T>(uih: &T, wall: Wall)
where
    T: UIHandler,
{
    draw_wall(uih, wall, LIGHT_GREEN);
}

/// Draws a preview of where a wall could go
pub fn draw_wall_placement_preview<T>(uih: &T, wall: Wall, player_id: u8)
where
    T: UIHandler,
{
    draw_wall(uih, wall, get_current_player_color(player_id));
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
/// This determines what can be done with this event (place wall, highlight player tile etc).
pub fn get_mouse_pointer_state(mouse_position: (f32, f32)) -> MouseState {
    if !is_mouse_inside_board(mouse_position) {
        return MouseState::NotInsideBoard;
    }

    let mut row: u16 = 0;
    let mut found_row: bool = false;
    let mut in_horizontal_wall_pos = false;
    let mut lane: u16 = 0;

    let mut col: u16 = 0;
    let mut found_col: bool = false;
    let mut in_vertical_wall_pos = false;

    for y in 0..NUM_TILES {
        let tile_base = GAME_GRID_BOX_START
            + (y as f32 * TILE_SIZE)
            + (y as f32 * WALL_WIDTH);

        if mouse_position.0 >= tile_base {
            if mouse_position.0 <= tile_base + TILE_SIZE {
                col = y as u16;
                found_col = true;
            } else if mouse_position.0 <= tile_base + TILE_SIZE + WALL_WIDTH {
                in_horizontal_wall_pos = true;
                lane = y as u16;
            }
        }

        // detect if it is in square
        if mouse_position.1 >= tile_base {
            if mouse_position.1 <= tile_base + TILE_SIZE {
                row = y as u16;
                found_row = true;
            } else if mouse_position.1 <= tile_base + TILE_SIZE + WALL_WIDTH {
                // detect if it is in wall
                in_vertical_wall_pos = true;
                lane = y as u16;
            }
        }
    }

    if found_row && found_col {
        MouseState::RowCol(Position {
            x: col as usize,
            y: row as usize,
        })
    } else if in_horizontal_wall_pos && in_vertical_wall_pos {
        MouseState::InBorderCrossSection
    } else {
        if found_row {
            return MouseState::WallPosition(Wall::new(
                Placement::new(row as usize, lane as usize),
                Orientation::Vertical,
            ));
        }

        MouseState::WallPosition(Wall::new(
            Placement::new(col as usize, lane as usize),
            Orientation::Horizontal,
        ))
    }
}

/// Draws a wall - wrapper method for the macroquad method
pub fn draw_wall<T>(uih: &T, b: Wall, c: Color)
where
    T: UIHandler,
{
    if b.placement.tile as u16 >= NUM_TILES as u16 - 1 {
        return;
    }

    let dim_1: f32 = GAME_GRID_BOX_START
        + (b.placement.tile as f32 * TILE_SIZE)
        + (b.placement.tile as f32 * WALL_WIDTH);
    let dim_2: f32 = GAME_GRID_BOX_START
        + ((b.placement.lane + 1) as f32 * TILE_SIZE)
        + (b.placement.lane as f32 * WALL_WIDTH);
    let long_side: f32 = (TILE_SIZE * 2.0) + WALL_WIDTH;

    let (x, y, w, h) = match b.orientation {
        Orientation::Horizontal => (dim_1, dim_2, long_side, WALL_WIDTH),
        Orientation::Vertical => (dim_2, dim_1, WALL_WIDTH, long_side),
    };
    uih.draw_rectangle(x, y, w, h, c);
}

/// Highlights player options around the player
pub fn highlight_player_options<T>(uih: &T, player_id: u8, pos: &Vec<Position>)
where
    T: UIHandler,
{
    const OFFSET: f32 = TILE_SIZE + WALL_WIDTH;

    for p in pos {
        uih.draw_rectangle(
            GAME_GRID_BOX_START + ((p.x as f32) * OFFSET),
            GAME_GRID_BOX_START + ((p.y as f32) * OFFSET),
            TILE_SIZE,
            TILE_SIZE,
            get_current_player_color(player_id),
        );
    }
}

/// Draws a preview of the player-tile
/// that the mouse is currently hovering over.
/// if it presents a valid move
pub fn draw_move_preview<T>(uih: &T, p: &Position)
where
    T: UIHandler,
{
    const OFFSET: f32 = TILE_SIZE + WALL_WIDTH;
    uih.draw_rectangle(
        GAME_GRID_BOX_START + ((p.x as f32) * OFFSET),
        GAME_GRID_BOX_START + ((p.y as f32) * OFFSET),
        TILE_SIZE,
        TILE_SIZE,
        Color::new(0.78, 0.48, 1.00, 0.80),
    )
}

#[cfg(test)]
mod tests {
    use crate::core::orientation::Orientation;
    use crate::core::placement::Placement;
    use crate::core::wall::Wall;
    use crate::ui::utils::*;

    use crate::core::position::Position;

    #[test]
    fn test_correct_num_of_precomputed_lines() {
        let pcl = GRID_LINES_ARR;
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
            MouseState::WallPosition(Wall::new(
                Placement::new(0, 0),
                Orientation::Horizontal
            ))
        );

        let mps = get_mouse_pointer_state((PADDING + TILE_SIZE + 1.0, PADDING));
        assert_eq!(
            mps,
            MouseState::WallPosition(Wall::new(
                Placement::new(0, 0),
                Orientation::Vertical
            ))
        );

        let mps = get_mouse_pointer_state((
            PADDING + TILE_SIZE + 1.0,
            PADDING + TILE_SIZE + 1.0,
        ));
        assert_eq!(mps, MouseState::InBorderCrossSection);
    }

    const TILE_IDX_0: f32 = 75.0;
    const TILE_IDX_1: f32 = 150.0;
    const TILE_IDX_2: f32 = 230.0;
    const TILE_IDX_3: f32 = 320.0;
    const TILE_IDX_4: f32 = 375.0;
    const TILE_IDX_5: f32 = 460.0;
    const TILE_IDX_6: f32 = 520.0;
    const TILE_IDX_7: f32 = 600.0;

    const WALL_LANE_0: f32 = 120.0;
    const WALL_LANE_1: f32 = 190.0;
    const WALL_LANE_2: f32 = 260.0;
    const WALL_LANE_3: f32 = 330.0;
    const WALL_LANE_4: f32 = 400.0;
    const WALL_LANE_5: f32 = 480.0;
    const WALL_LANE_6: f32 = 550.0;
    const WALL_LANE_7: f32 = 620.0;

    #[test]
    fn test_get_mouse_pointer_state_discover_all_tiles() {
        let tile_coordinates = [
            TILE_IDX_0, TILE_IDX_1, TILE_IDX_2, TILE_IDX_3, TILE_IDX_4,
            TILE_IDX_5, TILE_IDX_6, TILE_IDX_7,
        ];

        for x in 0..tile_coordinates.len() {
            for y in 0..tile_coordinates.len() {
                assert_eq!(
                    MouseState::RowCol(Position::new(x, y)),
                    get_mouse_pointer_state((
                        tile_coordinates[x],
                        tile_coordinates[y]
                    )),
                );
            }
        }
    }

    #[test]
    fn test_get_mouse_pointer_state_discover_all_walls_horizontal() {
        let tile_coordinates = [
            TILE_IDX_0, TILE_IDX_1, TILE_IDX_2, TILE_IDX_3, TILE_IDX_4,
            TILE_IDX_5, TILE_IDX_6, TILE_IDX_7,
        ];

        let lane_coordinates = [
            WALL_LANE_0,
            WALL_LANE_1,
            WALL_LANE_2,
            WALL_LANE_3,
            WALL_LANE_4,
            WALL_LANE_5,
            WALL_LANE_6,
            WALL_LANE_7,
        ];

        for (lane, lane_coordinate) in lane_coordinates.iter().enumerate() {
            for (tile, tile_coordinate) in tile_coordinates.iter().enumerate() {
                assert_eq!(
                    MouseState::WallPosition(Wall::new(
                        Placement::new(tile, lane),
                        Orientation::Horizontal
                    )),
                    get_mouse_pointer_state((
                        *tile_coordinate,
                        *lane_coordinate,
                    )),
                );
            }
        }
    }

    #[test]
    fn test_get_mouse_pointer_state_discover_all_walls_vertical() {
        let tile_coordinates = [
            TILE_IDX_0, TILE_IDX_1, TILE_IDX_2, TILE_IDX_3, TILE_IDX_4,
            TILE_IDX_5, TILE_IDX_6, TILE_IDX_7,
        ];

        let lane_coordinates = [
            WALL_LANE_0,
            WALL_LANE_1,
            WALL_LANE_2,
            WALL_LANE_3,
            WALL_LANE_4,
            WALL_LANE_5,
            WALL_LANE_6,
            WALL_LANE_7,
        ];

        for (lane, lane_coordinate) in lane_coordinates.iter().enumerate() {
            for (tile, tile_coordinate) in tile_coordinates.iter().enumerate() {
                assert_eq!(
                    MouseState::WallPosition(Wall::new(
                        Placement::new(tile, lane),
                        Orientation::Vertical
                    )),
                    get_mouse_pointer_state((
                        *lane_coordinate,
                        *tile_coordinate,
                    )),
                );
            }
        }
    }
}
