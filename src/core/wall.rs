use crate::core::orientation::Orientation;
use crate::core::placement::Placement;

/// Walls are used for walling player movement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wall {
    /// `orientation` is the wall's orientation
    pub orientation: Orientation,

    /// `placement` is the wall's placement on the grid
    pub placement: Placement,
}

impl Wall {
    pub fn new(placement: Placement, orientation: Orientation) -> Self {
        Self {
            orientation,
            placement,
        }
    }

    pub fn covers_placement(&self, x: usize, y: usize) -> bool {
        self.placement.lane == x
            && (self.placement.tile == y || self.placement.tile + 1 == y)
    }

    pub fn intersects(&self, w: &Wall) -> bool {
        match w.orientation {
            Orientation::Horizontal => {
                self.placement.tile == w.placement.lane
                    && self.placement.lane == w.placement.tile
            }
            Orientation::Vertical => {
                self.placement.tile == w.placement.lane
                    && self.placement.lane.abs_diff(w.placement.tile) == 0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::orientation::Orientation;
    use crate::core::placement::Placement;
    use crate::core::wall::Wall;

    #[test]
    fn test_not_intersect() {
        let b1 = Wall::new(Placement::new(0, 1), Orientation::Horizontal);
        let b2 = Wall::new(Placement::new(0, 1), Orientation::Vertical);
        let b3 = Wall::new(Placement::new(0, 0), Orientation::Vertical);

        assert!(!b2.intersects(&b3));
        assert!(!b3.intersects(&b2));
        assert!(!b1.intersects(&b2));
        assert!(!b2.intersects(&b1));
        assert!(!b1.intersects(&b3));
        assert!(!b3.intersects(&b1));
    }

    #[test]
    fn test_intersect() {
        let b1 = Wall::new(Placement::new(0, 0), Orientation::Horizontal);
        let b2 = Wall::new(Placement::new(0, 0), Orientation::Vertical);

        assert!(b1.intersects(&b2));
        assert!(b2.intersects(&b1));
    }
}
