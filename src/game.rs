use uuid::Uuid;

use crate::player::{Player, StartingPosition};

use crate::grid::Grid;

#[derive(Debug)]
pub struct Game<'a> {
    _id: String,
    _p1: Player,
    _p2: Player,
    pub g: Grid<'a>,
    // TODO dio : other stuff like date started etc
}

impl Game<'_> {
    pub fn new(n1: String, n2: String) -> Self {
        let _id = Uuid::new_v4().to_string();
        let _p1 = Player::new(n1, StartingPosition::Top);
        let _p2 = Player::new(n2, StartingPosition::Bottom);
        let g = Grid::new();
        Self { g, _p1, _p2, _id }
    }

    pub fn start(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::GRID_SIZE;
    #[test]
    fn simple_game() {
        let g = Game::new(String::from("a"), String::from("a"));
        assert_eq!(g._p1.position.y, 0);
        assert_eq!(g._p1.position.x, GRID_SIZE / 2);

        assert_eq!(g._p2.position.y, GRID_SIZE - 1);
        assert_eq!(g._p2.position.x, GRID_SIZE / 2);
    }
}
