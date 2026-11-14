use macroquad::color::BLACK;
use macroquad::prelude::{TextParams, draw_text_ex, get_text_center};
use macroquad::text::Font;

/// Font size for H1
pub const FONT_SIZE_H1: u16 = 60;

/// Font size for H2
pub const FONT_SIZE_H2: u16 = FONT_SIZE_H1 - 15u16;

/// Font size for H3
pub const FONT_SIZE_H3: u16 = FONT_SIZE_H1 - 30u16;

/// Padding for the box showing game stats
const GAME_STATS_BOX_PADDING: f32 = 5.0;

/// Convenience offset for where to start drawing the text in the stats box
const STATS_X_OFFSET: f32 = crate::ui::utils::GAME_STATS_BOX_START
    + (crate::ui::utils::GAME_STATS_BOX_WIDTH / 2.0);

pub struct TextPanel {}

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
            TextHeading::H1 => 12.0,
            TextHeading::H3 => 8.0,
            _ => 10.0,
        }
    }
}

impl TextPanel {
    pub fn draw_text(
        font: Option<&Font>,
        offset: f32,
        th: TextHeading,
        texts: Vec<String>,
    ) -> f32 {
        let params = TextParams {
            font,
            font_size: th.font_size(),
            color: BLACK,
            ..Default::default()
        };
        let mut offset = offset;
        texts.iter().for_each(|s| {
            let text_centre =
                get_text_center(s, font, params.font_size, 1.0, 0.0);

            offset += (text_centre.y.abs() + GAME_STATS_BOX_PADDING) * 2.0;

            draw_text_ex(
                s,
                STATS_X_OFFSET - text_centre.x,
                offset,
                params.clone(),
            );
        });

        offset + GAME_STATS_BOX_PADDING * th.bottom_offset_factor()
    }
}
