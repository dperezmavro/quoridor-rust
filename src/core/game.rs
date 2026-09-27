use crate::core::block::Block;
use crate::core::grid::Grid;
use crate::core::player::Player;
use crate::core::position::Position;

#[derive(Debug)]
pub struct Game {
    pub p1: Player,
    pub p2: Player,
    pub g: Grid,
    // TODO dio : other stuff like date started etc
}

impl Game {
    pub fn new(g: Grid, p1: Player, p2: Player) -> Self {
        Self { g, p1, p2 }
    }

    pub fn can_add_block(&mut self, b: Block) -> bool {
        self.g.can_place_block(&self.p1, &self.p2, b)
    }

    pub fn is_position_occupied_by_player(&self, p: &Position) -> bool {
        (self.p1.position.x == p.x && self.p1.position.y == p.y)
            || (self.p2.position.x == p.x && self.p2.position.y == p.y)
    }

    pub fn add_block(&mut self, b: Block) -> bool {
        self.g.place_block(&self.p1, &self.p2, b)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::core::block::Block;
    use crate::core::grid::Grid;
    use crate::core::orientation::Orientation;
    use crate::core::placement::Placement;
    use crate::core::player::StartingPosition;
    use crate::core::position::Position;
    use crate::core::tile::Tile;

    #[test]
    fn test_default() {
        let grid = Grid::new();
        let p1: Player = Player::new(String::from("a"), StartingPosition::Top);
        let p2: Player =
            Player::new(String::from("b"), StartingPosition::Bottom);
        let mut g = Game::new(grid, p1, p2);
        assert!(g.can_add_block(Block {
            orientation: Orientation::Vertical,
            placement: Placement { tile: 0, lane: 0 },
        }));
        assert!(g.add_block(Block {
            orientation: Orientation::Vertical,
            placement: Placement { tile: 0, lane: 0 },
        }));
    }

    #[test]
    fn test_cant_add_block_twice() {
        let grid = Grid::new();
        let p1: Player = Player::new(String::from("a"), StartingPosition::Top);
        let p2: Player =
            Player::new(String::from("b"), StartingPosition::Bottom);
        let mut g = Game::new(grid, p1, p2);
        let b = Block {
            orientation: Orientation::Vertical,
            placement: Placement { tile: 0, lane: 0 },
        };
        // test normal add
        assert!(g.can_add_block(b.clone()));
        assert!(g.add_block(b.clone()));
        //verify duplicate add rejected
        assert!(!g.can_add_block(b.clone()));
        assert!(!g.add_block(b.clone()));
    }

    #[test]
    fn test_is_position_occupied_by_player() {
        let grid = Grid::new();
        let p1: Player = Player::new(String::from("a"), StartingPosition::Top);
        let p2: Player =
            Player::new(String::from("b"), StartingPosition::Bottom);
        let mut g = Game::new(grid, p1, p2);

        assert!(g.is_position_occupied_by_player(&Position { x: 4, y: 0 }));
        assert!(g.is_position_occupied_by_player(&Position { x: 4, y: 8 }));

        g.p1.position = Position { x: 4, y: 6 };
        assert!(g.is_position_occupied_by_player(&Position { x: 4, y: 6 }));
    }

    #[test]
    fn test_ui_bugs() {
        let grid = Grid::new();
        let p1: Player = Player::new(String::from("a"), StartingPosition::Top);
        let p2: Player =
            Player::new(String::from("b"), StartingPosition::Bottom);
        let mut g = Game::new(grid, p1, p2);
        let b1 = Block {
            orientation: Orientation::Vertical,
            placement: Placement { tile: 0, lane: 3 },
        };
        let b2 = Block {
            orientation: Orientation::Vertical,
            placement: Placement { tile: 0, lane: 5 },
        };

        // bad block
        let b3 = Block {
            orientation: Orientation::Horizontal,
            placement: Placement { tile: 4, lane: 0 },
        };

        assert!(g.can_add_block(b1.clone()));
        assert!(g.add_block(b1.clone()));
        assert!(!g.can_add_block(b1.clone()));

        assert!(g.can_add_block(b2.clone()));
        assert!(g.add_block(b2.clone()));
        assert!(!g.can_add_block(b2.clone()));

        // this should fail
        assert!(!g.can_add_block(b3));
        assert!(!g.add_block(b3));

        let tile_vec = g.g.get_reachable_neighbour_tiles(&g.p1.position);
        assert_eq!(
            2,
            tile_vec.clone().iter().count(),
            "{:?}",
            tile_vec.clone().iter().collect::<Vec<&&Tile>>(),
        );
    }
}
