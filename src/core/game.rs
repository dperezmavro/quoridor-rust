use uuid::Uuid;

use crate::core::player::Player;

use crate::core::block::Block;
use crate::core::grid::Grid;

#[derive(Debug)]
pub struct Game {
    _id: String,
    pub p1: Player,
    pub p2: Player,
    pub g: Grid,
    // TODO dio : other stuff like date started etc
}

impl Game {
    pub fn new(g: Grid, p1: Player, p2: Player) -> Self {
        let _id = Uuid::new_v4().to_string();

        Self { g, p1, p2, _id }
    }

    pub fn can_add_block(&mut self, b: Block) -> bool {
        let res = self.g.can_place_block(&self.p1, &self.p2, b);
        if res {
            println!("can add block for {:?}: {:?}\n", self.p1, self.p2);
        }
        res
    }

    pub fn add_block(&mut self, b: Block) -> bool {
        self.g.place_block(&self.p1, &self.p2, b)
    }
}
