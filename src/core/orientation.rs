use serde::{Deserialize, Serialize};

/// Used to describe a wall's orientation
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default,
)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}
