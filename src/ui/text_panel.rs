use crate::ui::ui_handler::UIHandler;
use macroquad::color::BLACK;
use macroquad::prelude::{TextParams, get_text_center};
use macroquad::text::Font;

/// Font size for H1
const FONT_SIZE_H1: u16 = 60;

/// Font size for H2
const FONT_SIZE_H2: u16 = FONT_SIZE_H1 - 15u16;

/// Font size for H3
const FONT_SIZE_H3: u16 = FONT_SIZE_H1 - 30u16;
/// Padding after the text for H1
const BOTTOM_PADDING_H1: f32 = 8.0;

/// Padding after the text for H3
const BOTTOM_PADDING_H3: f32 = 4.0;

/// Padding after text for any other class
const BOTTOM_PADDING_OTHER: f32 = 6.0;

/// Padding for the box showing game stats
const GAME_STATS_BOX_PADDING: f32 = 5.0;

/// Convenience offset for where to start drawing the text in the stats box
const STATS_X_OFFSET: f32 = crate::ui::utils::GAME_STATS_BOX_START
    + (crate::ui::utils::GAME_STATS_BOX_WIDTH / 2.0);

#[derive(PartialEq, Clone, Copy)]
pub enum TextHeading {
    H1,
    H2,
    H3,
}

impl TextHeading {
    fn font_size(&self) -> u16 {
        match self {
            TextHeading::H1 => FONT_SIZE_H1,
            TextHeading::H2 => FONT_SIZE_H2,
            TextHeading::H3 => FONT_SIZE_H3,
        }
    }

    fn bottom_offset_factor(&self) -> f32 {
        match self {
            TextHeading::H1 => BOTTOM_PADDING_H1,
            TextHeading::H3 => BOTTOM_PADDING_H3,
            _ => BOTTOM_PADDING_OTHER,
        }
    }
}

pub struct TextPanel {
    offset: f32,
}

impl TextPanel {
    pub fn new(offset: f32) -> Self {
        Self {
            offset: offset + (2.0 * GAME_STATS_BOX_PADDING),
        }
    }
    pub fn draw_text<T>(
        &mut self,
        uih: &T,
        font: Option<&Font>,
        th: TextHeading,
        texts: Vec<&str>,
    ) where
        T: UIHandler,
    {
        let params = TextParams {
            font,
            font_size: th.font_size(),
            color: BLACK,
            ..Default::default()
        };

        texts.iter().enumerate().for_each(|(i, s)| {
            let text_centre =
                get_text_center(s, font, params.font_size, 1.0, 0.0);
            if th == TextHeading::H3 && i == 2 {
                self.offset += 10.0;
            }

            self.offset += (text_centre.y.abs() + GAME_STATS_BOX_PADDING) * 2.0;

            uih.draw_text_ex(
                s,
                STATS_X_OFFSET - text_centre.x,
                self.offset,
                params.clone(),
            );
        });

        self.offset += GAME_STATS_BOX_PADDING * th.bottom_offset_factor();
    }
}

#[cfg(test)]
mod tests {
    use crate::core::player::Player;
    use crate::ui::text_panel::*;
    use crate::ui::ui_handler::UIHandler;
    use macroquad::color::Color;
    use macroquad::prelude::TextParams;

    struct MockUiHandler {}

    impl UIHandler for MockUiHandler {
        async fn new() -> Self {
            Self {}
        }

        fn draw_game_base(
            &self,
            _remaining_walls: (usize, usize),
            _scores: (usize, usize),
        ) {
        }

        fn mouse_position(&self) -> (f32, f32) {
            (0.0, 0.0)
        }

        fn is_mouse_button_pressed(&self) -> bool {
            true
        }

        async fn next_frame(&self) {}

        fn draw_player(&self, _p: &Player, _highlight: bool) {}

        fn draw_rectangle(
            &self,
            _x: f32,
            _y: f32,
            _w: f32,
            _h: f32,
            _color: Color,
        ) {
        }

        fn draw_circle(&self, _x: f32, _y: f32, _r: f32, _color: Color) {}

        fn draw_text_ex(
            &self,
            _text: impl AsRef<str>,
            _x: f32,
            _y: f32,
            _params: TextParams,
        ) {
        }
    }

    #[test]
    fn assert_defaults() {
        assert_eq!(
            TextHeading::H2.bottom_offset_factor(),
            BOTTOM_PADDING_OTHER
        );

        assert_eq!(TextHeading::H1.bottom_offset_factor(), BOTTOM_PADDING_H1);

        assert_eq!(TextHeading::H3.bottom_offset_factor(), BOTTOM_PADDING_H3);

        let tp = TextPanel::new(0.0);
        assert_eq!(tp.offset, 2.0 * GAME_STATS_BOX_PADDING);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn assert_draw_text_h2() {
        let mui = MockUiHandler::new().await;
        let mut tp = TextPanel::new(0.0);
        let th = TextHeading::H2;
        tp.draw_text(&mui, None, th, vec![]);

        assert_eq!(
            tp.offset,
            GAME_STATS_BOX_PADDING * (2.0 + th.bottom_offset_factor())
        );
    }
}
