use crate::block::Block;
use crate::position::Position;
use uuid::Uuid;

pub enum Colour {
    White,
    Black,
}

pub struct Player {
    pub colour: Colour,
    pub name: String,
    pub id: String,
    pub position: Position,
    pub blocks: [Block; 10],
}

impl Player {
    pub fn new(name: String, colour: Colour, grid_size: u32) -> Self {
        let id = Uuid::new_v4().to_string();
        let p_x = match colour {
            Colour::White => 0,
            Colour::Black => grid_size - 1,
        };
        let blocks: [Block; 10] = core::array::from_fn(|i: usize| Block::new());
        Self {
            name,
            colour,
            id,
            position: Position::new(grid_size / 2, p_x), // multi-dimensional array, x and y are flipped around
            blocks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_white() {
        let p = Player::new("DUMMY".to_string(), Colour::White, 99);
        assert_eq!(p.position.x, 49);
        assert_eq!(p.position.y, 0);
    }

    fn test_player_black() {
        let p = Player::new("DUMMY".to_string(), Colour::Black, 99);
        assert_eq!(p.position.x, 49);
        assert_eq!(p.position.y, 99);
    }
}
