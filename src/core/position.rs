#[derive(Clone, Copy, Eq, Hash, Debug, Default, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
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

    pub fn is_valigned(&self, p: &Position) -> bool {
        self.x == p.x
    }

    pub fn is_haligned(&self, p: &Position) -> bool {
        self.y == p.y
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
        assert!(!p1.is_haligned(&p));

        let p1 = Position::new(1, 1);
        let p = Position::new(2, 1);
        assert!(p1.is_haligned(&p));
    }

    #[test]
    pub fn test_are_positions_valigned() {
        let p1 = Position::new(1, 1);
        let p = Position::new(2, 2);
        assert!(!p1.is_valigned(&p));

        let p1 = Position::new(1, 1);
        let p = Position::new(1, 2);
        assert!(p1.is_valigned(&p));
    }

    //
    // pub fn are_positions_valigned(&self, p1: &Position, p: &Position) -> bool {
    //     p1.x == p.x
    // }
    //
    // pub fn are_positions_haligned(&self, p1: &Position, p: &Position) -> bool {
    //     p1.y == p.y
    // }
}
