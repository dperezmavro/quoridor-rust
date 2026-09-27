use crate::core::grid::GRID_SIZE;
use crate::core::position::Position;
use macroquad::color::{BLUE, Color, YELLOW};

pub const NUM_BLOCKS: usize = 10;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum StartingPosition {
    Top,
    Bottom,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Player {
    pub starting_position: StartingPosition,
    pub name: String,
    pub position: Position,
    pub remaining_blocks: usize,
}

impl Player {
    pub fn new(name: String, sp: StartingPosition) -> Self {
        let p_y = match sp {
            StartingPosition::Top => 0,
            StartingPosition::Bottom => GRID_SIZE - 1,
        };
        Self {
            name,
            starting_position: sp,
            position: Position::new(GRID_SIZE / 2, p_y),
            remaining_blocks: NUM_BLOCKS,
        }
    }

    pub fn get_y_goal(&self) -> usize {
        match self.starting_position {
            StartingPosition::Top => GRID_SIZE - 1,
            StartingPosition::Bottom => 0,
        }
    }

    pub fn get_color(&self) -> Color {
        if self.starting_position == StartingPosition::Top {
            return YELLOW;
        }
        BLUE
    }

    pub fn consume_block(&mut self) {
        self.remaining_blocks -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_white() {
        let p = Player::new("DUMMY".to_string(), StartingPosition::Top);
        assert_eq!(p.position.x, GRID_SIZE / 2);
        assert_eq!(p.position.y, 0);
    }

    #[test]
    fn test_player_black() {
        let p = Player::new("DUMMY".to_string(), StartingPosition::Bottom);
        assert_eq!(p.position.x, GRID_SIZE / 2);
        assert_eq!(p.position.y, GRID_SIZE - 1);
    }

    #[test]
    fn test_player_blocks() {
        let p = Player::new("DUMMY".to_string(), StartingPosition::Bottom);
        assert_eq!(p.remaining_blocks, NUM_BLOCKS);
    }
}
