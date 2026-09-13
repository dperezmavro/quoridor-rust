use uuid::Uuid;

use crate::player::{Player, StartingPosition};

use crate::block::Block;
use crate::grid::Grid;

#[derive(Debug)]
pub struct Game<'a> {
    _id: String,
    _p1: Player,
    _p2: Player,
    pub g: &'a mut Grid<'a>,
    // TODO dio : other stuff like date started etc
}

impl<'a> Game<'a> {
    pub fn new(n1: String, n2: String, g: &'a mut Grid<'a>) -> Self {
        let _id = Uuid::new_v4().to_string();
        let _p1 = Player::new(n1, StartingPosition::Top);
        let _p2 = Player::new(n2, StartingPosition::Bottom);

        Self { g, _p1, _p2, _id }
    }

    pub fn start(&mut self) {}

    pub fn add_block(&mut self, b: &'a Block) -> bool {
        // self.g.place_block(&self._p1, &self._p2, b)
        self.g.place_block(&self._p1, &self._p2, b)
    }
}
