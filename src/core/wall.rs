use crate::core::orientation::Orientation;
use crate::core::placement::Placement;

/// Walls are used for walling player movement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wall {
    /// `orientation` is the wall's orientation
    pub orientation: Orientation,

    /// `placement` is the wall's placement on the grid
    pub placement: Placement,
}

impl Wall {
    pub fn new(placement: Placement, orientation: Orientation) -> Self {
        Self {
            orientation,
            placement,
        }
    }
}
