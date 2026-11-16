use crate::core::grid::NUM_TILES;
use crate::core::player_turn::PlayerTurn;
use crate::core::position::Position;
use serde::{Deserialize, Serialize};

/// Number of walls per player
pub const NUM_WALLS: usize = 10;

/// Describes the player, their position and their remaining blocks
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Player {
    pub id: PlayerTurn,
    pub name: String,
    pub position: Position,
    pub remaining_walls: usize,
}

const ROW_INDEX_FIRST: u8 = 0;
const ROW_INDEX_LAST: u8 = NUM_TILES - 1;

impl Player {
    pub fn new(name: String, id: PlayerTurn) -> Self {
        let p_y = match id {
            PlayerTurn::One => ROW_INDEX_FIRST,
            PlayerTurn::Two => ROW_INDEX_LAST,
        };
        Self {
            name,
            id,
            position: Position::new(NUM_TILES / 2, p_y),
            remaining_walls: NUM_WALLS,
        }
    }

    pub fn goal_row(&self) -> u8 {
        match self.id {
            PlayerTurn::One => NUM_TILES - 1,
            PlayerTurn::Two => 0,
        }
    }

    pub fn is_there(&self, position: &Position) -> bool {
        self.position == *position
    }

    pub fn is_neighbour(&self, p: &Player) -> bool {
        self.position.is_neighbour(&p.position)
    }

    pub fn consume_wall(&mut self) {
        self.remaining_walls -= 1;
    }

    pub fn has_remaining_walls(&self) -> bool {
        self.remaining_walls > 0
    }

    pub fn is_x_aligned(&self, p: &Player) -> bool {
        self.position.is_x_aligned(&p.position)
    }

    pub fn is_y_aligned(&self, p: &Player) -> bool {
        self.position.is_y_aligned(&p.position)
    }

    pub fn is_above(&self, p: &Player) -> bool {
        self.position.is_above(&p.position)
    }

    pub fn is_left_of(&self, p: &Player) -> bool {
        self.position.is_left_of(&p.position)
    }

    pub fn can_jump_over(&self, p: &Player, pos: &Position) -> bool {
        self.is_neighbour(p)
            && self.is_x_aligned(p)
            && (self.is_above(p) && p.position.is_above(pos))
            || (p.is_above(self) && pos.is_above(&p.position))
    }

    pub fn can_cross_over(&self, p: &Player, pos: &Position) -> bool {
        self.is_neighbour(p)
            && self.is_y_aligned(p)
            && (self.is_left_of(p) && p.position.is_left_of(pos))
            || (p.is_left_of(self) && pos.is_left_of(&p.position))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_white() {
        let p = Player::new("DUMMY".to_string(), PlayerTurn::One);
        assert_eq!(p.position.x, NUM_TILES / 2);
        assert_eq!(p.position.y, 0);
    }

    #[test]
    fn test_can_cross_over() {
        let mut p1 = Player::new("DUMMY".to_string(), PlayerTurn::One);
        let mut p2 = Player::new("DUMMY".to_string(), PlayerTurn::Two);

        p1.position = Position::new(3, 4);
        p2.position = Position::new(4, 4);

        assert!(p1.can_cross_over(&p2, &Position::new(5, 4)));
        assert!(p2.can_cross_over(&p1, &Position::new(2, 4)));
    }

    #[test]
    fn test_can_jump_over() {
        let mut p1 = Player::new("DUMMY".to_string(), PlayerTurn::One);
        let mut p2 = Player::new("DUMMY".to_string(), PlayerTurn::Two);

        p1.position = Position::new(4, 3);
        p2.position = Position::new(4, 4);

        assert!(p1.can_jump_over(&p2, &Position::new(4, 5)));
        assert!(p2.can_jump_over(&p1, &Position::new(4, 2)));
    }

    #[test]
    fn test_player_position() {
        let p = Player::new("DUMMY".to_string(), PlayerTurn::Two);
        assert_eq!(p.position.x, NUM_TILES / 2);
        assert_eq!(p.position.y, NUM_TILES - 1);
    }

    #[test]
    fn test_player_walls() {
        let p = Player::new("DUMMY".to_string(), PlayerTurn::Two);
        assert_eq!(p.remaining_walls, NUM_WALLS);
    }

    #[test]
    pub fn test_are_positions_haligned() {
        let mut p1 = Player::new(String::from("DUMMY"), PlayerTurn::One);
        p1.position = Position::new(1, 1);
        let mut p2 = p1.clone();
        p2.position = Position::new(2, 2);

        assert!(!p1.is_y_aligned(&p2));

        p2.position = Position::new(2, 1);
        assert!(p1.is_y_aligned(&p2));
    }

    #[test]
    pub fn test_is_there() {
        let mut p1 = Player::new(String::from("DUMMY"), PlayerTurn::One);
        assert!(!p1.is_there(&Position::new(1, 1)));
        p1.position = Position::new(1, 1);
        assert!(p1.is_there(&Position::new(1, 1)));
    }

    #[test]
    pub fn test_are_positions_valigned() {
        let mut p1 = Player::new(String::from("DUMMY"), PlayerTurn::One);
        p1.position = Position::new(1, 1);
        let mut p2 = p1.clone();
        p2.position = Position::new(2, 2);

        assert!(!p1.is_x_aligned(&p2));

        p2.position = Position::new(1, 2);
        assert!(p1.is_x_aligned(&p2));
    }

    #[test]
    pub fn test_are_positions_above() {
        let mut p1 = Player::new(String::from("DUMMY"), PlayerTurn::One);
        p1.position = Position::new(1, 1);
        let mut p2 = p1.clone();
        p2.position = Position::new(2, 2);
        assert!(!p1.is_above(&p2));

        p2.position = Position::new(1, 2);
        assert!(!p1.is_above(&p2));
        assert!(p2.is_above(&p1));
    }
}
