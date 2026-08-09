use uuid::Uuid;


use crate::player::{
    Colour,
    Player,
};

use crate::grid::Grid;

pub struct Game {
    id: String,
    pub p1: Player,
    pub p2: Player,
    pub g: Grid,
    // TODO dio : other stuff like date started etc
}


impl Game {
    pub fn new(
        n1: String,
        n2: String,
        size: usize,
    ) -> Self {
        let id = Uuid::new_v4().to_string();

        Self {
            g: Grid::new(size),
            p1: Player::new(n1, Colour::White, size),
            p2: Player::new(n2, Colour::Black, size),
            id,
        }
    }
}
