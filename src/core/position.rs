use serde::{Deserialize, Serialize};

/// Generic position object
#[derive(
    Serialize, Deserialize, Clone, Copy, Eq, Hash, Debug, Default, PartialEq,
)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn is_neighbour(&self, p: &Position) -> bool {
        if self.x == p.x || self.y == p.y {
            return ((self.x as i32 - p.x as i32).abs()
                + (self.y as i32 - p.y as i32).abs())
                == 1;
        }
        false
    }

    pub fn is_x_aligned(&self, p: &Position) -> bool {
        self.x == p.x
    }

    pub fn is_y_aligned(&self, p: &Position) -> bool {
        self.y == p.y
    }

    pub fn is_above(&self, p: &Position) -> bool {
        self.y > p.y && self.x == p.x
    }

    pub fn is_left_of(&self, p: &Position) -> bool {
        self.x < p.x && self.y == p.y
    }
}

#[cfg(test)]
mod tests {
    use crate::core::position::Position;

    #[test]
    pub fn test_are_positions_neighbours() {
        let p1 = Position::new(1, 1);
        let p = Position::new(2, 2);
        assert!(!p1.is_neighbour(&p));

        let p1 = Position::new(1, 1);
        let p = Position::new(2, 1);
        assert!(p1.is_neighbour(&p));

        let p1 = Position::new(1, 1);
        let p = Position::new(1, 2);
        assert!(p1.is_neighbour(&p));

        let p1 = Position::new(2, 1);
        let p = Position::new(2, 2);
        assert!(p1.is_neighbour(&p));

        let p1 = Position::new(1, 2);
        let p = Position::new(2, 2);
        assert!(p1.is_neighbour(&p));
    }

    #[test]
    pub fn test_are_positions_haligned() {
        let p1 = Position::new(1, 1);
        let p = Position::new(2, 2);
        assert!(!p1.is_y_aligned(&p));

        let p1 = Position::new(1, 1);
        let p = Position::new(2, 1);
        assert!(p1.is_y_aligned(&p));
    }

    #[test]
    pub fn test_are_positions_valigned() {
        let p1 = Position::new(1, 1);
        let p = Position::new(2, 2);
        assert!(!p1.is_x_aligned(&p));

        let p1 = Position::new(1, 1);
        let p = Position::new(1, 2);
        assert!(p1.is_x_aligned(&p));
    }

    #[test]
    pub fn test_are_positions_above() {
        let p1 = Position::new(1, 1);
        let p2 = Position::new(2, 2);
        assert!(!p1.is_above(&p2));

        let p1 = Position::new(1, 1);
        let p2 = Position::new(1, 2);
        assert!(!p1.is_above(&p2));
        assert!(p2.is_above(&p1));
    }
}
