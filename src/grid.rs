pub struct Grid {
    pub size: u32,
}

enum TileType {
    BlockPosition,
    BlockNode,
    Playable,
}

enum PlayableTileState {
    PlayerOccupied {player_id: String},
    Free,
}

impl Grid {
    pub fn new(size: u32) -> Self {
        Self { size }
    }
}
