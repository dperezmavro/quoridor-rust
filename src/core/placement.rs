use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Placement {
    pub tile: u8,
    pub lane: u8,
}

impl Placement {
    pub fn new(tile: u8, lane: u8) -> Self {
        Self { tile, lane }
    }
}
