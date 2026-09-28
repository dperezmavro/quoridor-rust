#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Placement {
    pub tile: usize,
    pub lane: usize,
}

impl Placement {
    pub fn new(tile: usize, lane: usize) -> Self {
        Self { tile, lane }
    }
}
