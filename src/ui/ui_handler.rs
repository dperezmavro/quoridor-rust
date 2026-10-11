use crate::core::player::Player;
use macroquad::color::Color;

pub trait UIHandler {
    async fn new() -> Self;
    fn draw_game_base(&self, remaining_walls: (usize, usize));

    fn mouse_position(&self) -> (f32, f32);

    fn is_mouse_button_pressed(&self) -> bool;

    async fn next_frame(&self);

    fn draw_player(&self, p: &Player, highlight: bool);

    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color);

    fn draw_circle(&self, x: f32, y: f32, r: f32, color: Color);
}
