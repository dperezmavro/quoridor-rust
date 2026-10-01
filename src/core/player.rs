use crate::core::grid::NUM_TILES;
use crate::core::position::Position;

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
            StartingPosition::Bottom => NUM_TILES - 1,
        };
        Self {
            name,
            starting_position: sp,
            position: Position::new(NUM_TILES / 2, p_y),
            remaining_blocks: NUM_BLOCKS,
        }
    }

    pub fn get_y_goal(&self) -> usize {
        match self.starting_position {
            StartingPosition::Top => NUM_TILES - 1,
            StartingPosition::Bottom => 0,
        }
    }

    pub fn is_there(&self, position: &Position) -> bool {
        self.position == *position
    }

    pub fn consume_block(&mut self) {
        self.remaining_blocks -= 1;
    }

    pub fn has_remaining_blocks(&self) -> bool {
        self.remaining_blocks > 0
    }

    pub fn is_valigned(&self, p: &Player) -> bool {
        self.position.is_valigned(&p.position)
    }

    pub fn is_haligned(&self, p: &Player) -> bool {
        self.position.is_haligned(&p.position)
    }

    pub fn is_above(&self, p: &Player) -> bool {
        self.position.is_above(&p.position)
    }

    pub fn is_left_of(&self, p: &Player) -> bool {
        self.position.is_left_of(&p.position)
    }

    pub fn can_jump_over(&self, p: &Player, pos: &Position) -> bool {
        self.is_valigned(p)
            && ((self.is_above(p) && p.position.is_above(pos))
                || (p.is_above(self) && pos.is_above(&p.position)))
    }

    pub fn can_cross_over(&self, p: &Player, pos: &Position) -> bool {
        self.is_haligned(p)
            && (self.is_left_of(p) && p.position.is_left_of(pos))
            || (p.is_left_of(self) && pos.is_left_of(&p.position))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_white() {
        let p = Player::new("DUMMY".to_string(), StartingPosition::Top);
        assert_eq!(p.position.x, NUM_TILES / 2);
        assert_eq!(p.position.y, 0);
    }

    #[test]
    fn test_player_black() {
        let p = Player::new("DUMMY".to_string(), StartingPosition::Bottom);
        assert_eq!(p.position.x, NUM_TILES / 2);
        assert_eq!(p.position.y, NUM_TILES - 1);
    }

    #[test]
    fn test_player_blocks() {
        let p = Player::new("DUMMY".to_string(), StartingPosition::Bottom);
        assert_eq!(p.remaining_blocks, NUM_BLOCKS);
    }

    #[test]
    pub fn test_are_positions_haligned() {
        let mut p1 = Player::new(String::from("DUMMY"), StartingPosition::Top);
        p1.position = Position::new(1, 1);
        let mut p2 = p1.clone();
        p2.position = Position::new(2, 2);

        assert!(!p1.is_haligned(&p2));

        p2.position = Position::new(2, 1);
        assert!(p1.is_haligned(&p2));
    }

    #[test]
    pub fn test_is_there() {
        let mut p1 = Player::new(String::from("DUMMY"), StartingPosition::Top);
        assert!(!p1.is_there(&Position::new(1, 1)));
        p1.position = Position::new(1, 1);
        assert!(p1.is_there(&Position::new(1, 1)));
    }

    #[test]
    pub fn test_are_positions_valigned() {
        let mut p1 = Player::new(String::from("DUMMY"), StartingPosition::Top);
        p1.position = Position::new(1, 1);
        let mut p2 = p1.clone();
        p2.position = Position::new(2, 2);

        assert!(!p1.is_valigned(&p2));

        p2.position = Position::new(1, 2);
        assert!(p1.is_valigned(&p2));
    }

    #[test]
    pub fn test_are_positions_above() {
        let mut p1 = Player::new(String::from("DUMMY"), StartingPosition::Top);
        p1.position = Position::new(1, 1);
        let mut p2 = p1.clone();
        p2.position = Position::new(2, 2);
        assert!(!p1.is_above(&p2));

        p2.position = Position::new(1, 2);
        assert!(!p1.is_above(&p2));
        assert!(p2.is_above(&p1));
    }
}
