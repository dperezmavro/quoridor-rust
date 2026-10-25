use crate::core::orientation::Orientation;
use crate::core::placement::Placement;
use serde::Serialize;

/// Walls are used for walling player movement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
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
        let w1 = Wall::new(Placement::new(0, 0), Orientation::Horizontal);
        let w2 = Wall::new(Placement::new(0, 0), Orientation::Vertical);

        assert!(w1.intersects(&w2));
        assert!(w2.intersects(&w1));
    }

    #[test]
    fn test_t_junction() {
        let w1 = Wall::new(Placement::new(4, 6), Orientation::Horizontal);
        let intersecting_wall =
            Wall::new(Placement::new(6, 4), Orientation::Vertical);

        // construct a grid of walls around the horizontal block
        for tile in 5..=7 {
            for lane in 3..=5 {
                let w = Wall::new(
                    Placement::new(tile, lane),
                    Orientation::Vertical,
                );
                if w == intersecting_wall {
                    assert!(w1.intersects(&w), "intersects for wall {:?}", w);
                } else {
                    assert!(!w1.intersects(&w), "intersects for wall {:?}", w);
                }
            }
        }
    }
}
