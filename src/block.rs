use crate::placement::Placement;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Block {
    pub orientation: Orientation,
    pub placement: Placement,
}

impl Block {
    pub fn new(placement: Placement, orientation: Orientation) -> Self {
        Self {
            orientation,
            placement,
        }
    }
}
