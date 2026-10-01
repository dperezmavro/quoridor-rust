use crate::core::orientation::Orientation;
use crate::core::placement::Placement;

/// Blocks are used for blocking player movement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wall {
    /// `orientation` is the block's orientation
    pub orientation: Orientation,

    /// `placement` is the block's placement on the grid
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
