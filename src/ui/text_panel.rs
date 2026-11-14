use macroquad::text::Font;

/// Font size for H1
pub const FONT_SIZE_H1: u16 = 60;

/// Font size for H2
pub const FONT_SIZE_H2: u16 = FONT_SIZE_H1 - 15u16;

/// Font size for H3
pub const FONT_SIZE_H3: u16 = FONT_SIZE_H1 - 30u16;

pub struct TextPanel<'a> {
    font: Option<&'a Font>,
    existing_offset: f32,
}

pub enum TextHeading {
    H1,
    H2,
    H3,
}

impl TextHeading {
    fn font(&self) -> u16 {
        match self {
            TextHeading::H1 => FONT_SIZE_H1,
            TextHeading::H2 => FONT_SIZE_H2,
            TextHeading::H3 => FONT_SIZE_H3,
        }
    }
}

impl<'a> TextPanel<'a> {
    fn draw_text(&self, th: TextHeading, texts: Vec<String>) {}
}
