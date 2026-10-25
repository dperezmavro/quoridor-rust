use serde::Serialize;

/// Used to describe a wall's orientation
#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}
