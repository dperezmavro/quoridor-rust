use crate::core::player::Player;
use macroquad::color::Color;
use macroquad::prelude::TextParams;

pub trait UIHandler {
    async fn new() -> Self;

    // Convenience function to draw the background, game grid
    // placed blocks and score information
    fn draw_game_base(
        &self,
        remaining_walls: (usize, usize),
        scores: (usize, usize),
    );

    // Returns the mouse's x,y coordinates
    fn mouse_position(&self) -> (f32, f32);

    // Checks whether the left moust button is pressed
    fn is_mouse_button_pressed(&self) -> bool;

    // Needed by MacroQuad for rendering
    async fn next_frame(&self);

    // Draws the player on the grid
    fn draw_player(&self, p: &Player, highlight: bool);

    // Convenience function to wrap macroquad's draw_rectangle
    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color);

    // Convenience function to wrap macroquad's draw_circle
    fn draw_circle(&self, x: f32, y: f32, r: f32, color: Color);

    fn draw_text_ex(
        &self,
        text: impl AsRef<str>,
        x: f32,
        y: f32,
        params: TextParams,
    );

    fn get_text_center(
        &self,
        text: impl AsRef<str>,
        font: Option<&macroquad::text::Font>,
        font_size: u16,
        font_scale: f32,
        rotation: f32,
    ) -> macroquad::math::Vec2;
}
