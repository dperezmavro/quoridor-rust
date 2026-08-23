use crate::block::Block;
use crate::grid::GRID_SIZE;
use crate::position::Position;
use uuid::Uuid;

const NUM_BLOCKS: usize = 10;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Colour {
    White,
    Black,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Player {
    pub colour: Colour,
    pub name: String,
    pub id: String,
    pub position: Position,
    pub blocks: [Block; NUM_BLOCKS],
}

impl Player {
    pub fn new(name: String, colour: Colour) -> Self {
        let id = Uuid::new_v4().to_string();
        let p_y = match colour {
            Colour::White => 0,
            Colour::Black => GRID_SIZE - 1,
        };
        let blocks: [Block; NUM_BLOCKS] = [Block::default(); NUM_BLOCKS];
        Self {
            name,
            colour,
            id,
            position: Position::new(GRID_SIZE / 2, p_y),
            blocks,
        }
    }

    pub fn set_position(&mut self, p: Position) {
        self.position = p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_white() {
        let p = Player::new("DUMMY".to_string(), Colour::White);
        assert_eq!(p.position.x, GRID_SIZE / 2);
        assert_eq!(p.position.y, 0);
    }

    #[test]
    fn test_player_black() {
        let p = Player::new("DUMMY".to_string(), Colour::Black);
        assert_eq!(p.position.x, GRID_SIZE / 2);
        assert_eq!(p.position.y, GRID_SIZE - 1);
    }

    #[test]
    fn test_player_blocks() {
        let p = Player::new("DUMMY".to_string(), Colour::Black);
        assert_eq!(p.blocks.len(), NUM_BLOCKS);
    }
}
