use crate::core::orientation::Orientation;
use crate::core::player::{NUM_WALLS, Player};
use crate::core::position::Position;
use crate::core::wall::Wall;
use std::collections::HashSet;

pub const NUM_TILES: usize = 9;

#[derive(Debug)]
pub struct Grid {
    pub player_tiles: Box<[Box<[Position; NUM_TILES]>; NUM_TILES]>,
    horizontal_walls: Vec<Wall>,
    vertical_walls: Vec<Wall>,
}

fn generate_row(y: usize) -> Box<[Position; NUM_TILES]> {
    let mut res = Box::new([Position::default(); NUM_TILES]);
    for i in 0..NUM_TILES {
        res[i] = Position { x: i, y };
    }

    res
}

impl Grid {
    pub fn new() -> Self {
        let player_1_pos = Position {
            y: 0,
            x: NUM_TILES / 2,
        };
        let player_2_pos = Position {
            y: NUM_TILES - 1,
            x: NUM_TILES / 2,
        };
        let mut first_row = generate_row(0);
        first_row[NUM_TILES / 2] = player_1_pos;
        let mut last_row = generate_row(NUM_TILES - 1);
        last_row[NUM_TILES / 2] = player_2_pos;

        let player_tiles: Box<[Box<[Position; NUM_TILES]>; NUM_TILES]> =
            Box::new([
                first_row,
                generate_row(1),
                generate_row(2),
                generate_row(3),
                generate_row(4),
                generate_row(5),
                generate_row(6),
                generate_row(7),
                last_row,
            ]);

        Self {
            player_tiles,
            horizontal_walls: Vec::with_capacity(NUM_WALLS),
            vertical_walls: Vec::with_capacity(NUM_WALLS),
        }
    }

    pub fn get_unoccupied_reachable_neighbour_positions(
        &self,
        p1_position: &Position,
        p2_position: &Position,
    ) -> Vec<Position> {
        self.get_reachable_neighbour_positions(p1_position)
            .into_iter()
            .filter(|p| p != p1_position && p != p2_position)
            .collect()
    }

    fn positions_north(&self, pos: &Position) -> bool {
        pos.y > 0
            && self
                .horizontal_walls
                .iter()
                .all(|w| !w.covers_placement(pos.y - 1, pos.x))
    }

    fn positions_south(&self, pos: &Position) -> bool {
        pos.y < NUM_TILES - 1
            && self
                .horizontal_walls
                .iter()
                .all(|w| !w.covers_placement(pos.y, pos.x))
    }

    fn tiles_west(&self, pos: &Position) -> bool {
        pos.x > 0
            && self
                .vertical_walls
                .iter()
                .all(|w| !w.covers_placement(pos.x - 1, pos.y))
    }

    fn tiles_east(&self, pos: &Position) -> bool {
        pos.x < NUM_TILES - 1
            && self
                .vertical_walls
                .iter()
                .all(|w| !w.covers_placement(pos.x, pos.y))
    }

    // Used for finding out all available neighbours
    fn get_reachable_neighbour_positions(&self, p: &Position) -> Vec<Position> {
        let mut available_positions: Vec<Position> = Vec::new();

        if self.positions_north(p) {
            available_positions.push(Position::new(p.x, p.y - 1));
        }

        if self.positions_south(p) {
            available_positions.push(Position::new(p.x, p.y + 1));
        }

        if self.tiles_west(p) {
            available_positions.push(Position::new(p.x - 1, p.y));
        }

        if self.tiles_east(p) {
            available_positions.push(Position::new(p.x + 1, p.y));
        }

        available_positions
    }

    pub fn can_player_reach_goal(&self, p: &Player) -> bool {
        let mut visited: HashSet<Position> = HashSet::new();
        let mut positions_for_exploring: HashSet<Position> = HashSet::new();

        positions_for_exploring.insert(p.position.clone());

        while !positions_for_exploring.is_empty() {
            let curr_pos =
                positions_for_exploring.iter().cloned().next().unwrap();
            if visited.contains(&curr_pos) {
                continue;
            }

            visited.insert(curr_pos.clone());
            let neighbors = self.get_reachable_neighbour_positions(&curr_pos);

            for pos in neighbors {
                if pos.y == p.goal_row() {
                    return true;
                }

                if !visited.contains(&pos) {
                    positions_for_exploring.insert(pos.clone());
                }
            }

            positions_for_exploring.remove(&curr_pos);
        }

        false
    }

    fn pop_last_wall(&mut self, b: &Wall) {
        match b.orientation {
            Orientation::Horizontal => {
                self.horizontal_walls.pop().unwrap();
            }
            Orientation::Vertical => {
                self.vertical_walls.pop().unwrap();
            }
        };
    }

    fn intersects_existing_walls(&self, wall: &Wall) -> bool {
        match wall.orientation {
            Orientation::Horizontal => {
                self.vertical_walls.iter().any(|w| w.intersects(wall))
            }
            Orientation::Vertical => {
                self.horizontal_walls.iter().any(|w| w.intersects(wall))
            }
        }
    }

    fn add_wall(&mut self, b: Wall) {
        match b.orientation {
            Orientation::Horizontal => {
                self.horizontal_walls.push(b);
            }
            Orientation::Vertical => {
                self.vertical_walls.push(b);
            }
        }
    }

    fn is_wall_position_free(&self, b: &Wall) -> bool {
        let target_list = match b.orientation {
            Orientation::Horizontal => &self.horizontal_walls,
            Orientation::Vertical => &self.vertical_walls,
        };
        target_list.iter().all(|t| {
            !(t.placement.lane == b.placement.lane
                && b.placement.tile.abs_diff(t.placement.tile) <= 1)
        })
    }

    pub fn place_wall(&mut self, p1: &Player, p2: &Player, b: Wall) -> bool {
        if self.can_place_wall(p1, p2, b) {
            self.add_wall(b);
            return true;
        }
        false
    }

    pub fn can_place_wall(
        &mut self,
        p1: &Player,
        p2: &Player,
        b: Wall,
    ) -> bool {
        if b.placement.tile >= NUM_TILES - 1
            || b.placement.lane >= NUM_TILES - 1
        {
            return false;
        }
        if !self.intersects_existing_walls(&b) && self.is_wall_position_free(&b)
        {
            self.add_wall(b);

            let res = self.can_player_reach_goal(p1)
                && self.can_player_reach_goal(p2);
            self.pop_last_wall(&b);
            return res;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::placement::Placement;
    use crate::core::player::StartingPosition;

    /********************************************/
    //         place_wall
    /********************************************/
    #[test]
    fn test_place_wall_out_of_bounds() {
        let tests = [
            Wall::new(Placement::new(8, 0), Orientation::Horizontal),
            Wall::new(Placement::new(0, 8), Orientation::Horizontal),
        ];
        for b in tests.iter() {
            let mut grid = Grid::new();
            let p1 = Player::new(String::new(), StartingPosition::Top);
            let p2 = Player::new(String::new(), StartingPosition::Bottom);
            let success = grid.place_wall(&p1, &p2, *b);
            assert!(!success);
        }
    }

    #[test]
    fn place_wall_explicit() {
        let mut grid = Grid::new();
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);

        let b1 = Wall::new(Placement::new(0, 0), Orientation::Horizontal);
        assert!(grid.place_wall(&p1, &p2, b1));

        let b2 = Wall::new(Placement::new(1, 0), Orientation::Horizontal);
        assert!(!grid.place_wall(&p1, &p2, b2));

        let b3 = Wall::new(Placement::new(3, 0), Orientation::Horizontal);
        assert!(grid.place_wall(&p1, &p2, b3));
        assert!(!grid.place_wall(&p1, &p2, b3));
    }

    #[test]
    fn test_bulk_place_walls_illegal() {
        let tests = [
            Wall::new(Placement::new(0, 3), Orientation::Horizontal),
            Wall::new(Placement::new(2, 3), Orientation::Horizontal),
            Wall::new(Placement::new(4, 3), Orientation::Horizontal),
            Wall::new(Placement::new(6, 3), Orientation::Horizontal),
            Wall::new(Placement::new(7, 5), Orientation::Horizontal),
        ];

        let mut g = Grid::new();
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);
        let results: bool = tests.iter().all(|b| g.place_wall(&p1, &p2, *b));
        assert!(results);

        // wall that makes the grid illegal
        let b = Wall::new(Placement::new(4, 6), Orientation::Vertical);
        assert!(!g.can_place_wall(&p1, &p2, b));
        assert!(!g.place_wall(&p1, &p2, b));
    }
    #[test]
    fn test_bulk_place_walls_legal() {
        let tests = [vec![
            Wall::new(Placement::new(0, 3), Orientation::Horizontal),
            Wall::new(Placement::new(2, 3), Orientation::Horizontal),
            Wall::new(Placement::new(4, 3), Orientation::Horizontal),
            Wall::new(Placement::new(6, 3), Orientation::Horizontal),
            Wall::new(Placement::new(7, 5), Orientation::Horizontal),
        ]];

        for t in tests.iter() {
            let mut g = Grid::new();
            let p1 = Player::new(String::new(), StartingPosition::Top);
            let p2 = Player::new(String::new(), StartingPosition::Bottom);

            let results: bool = t.iter().all(|b| g.place_wall(&p1, &p2, *b));
            assert!(results);
        }
    }

    /********************************************/
    //         get_reachable_neighbour_tiles
    /********************************************/
    #[test]
    fn get_reachable_neighbour_tiles() {
        struct TestCase(usize, usize, usize);
        let tests = vec![
            TestCase(0, 0, 2),
            TestCase(1, 1, 4),
            TestCase(NUM_TILES - 1, NUM_TILES - 1, 2),
            TestCase(0, NUM_TILES - 1, 2),
            TestCase(NUM_TILES - 1, 0, 2),
            TestCase(NUM_TILES - 3, NUM_TILES - 3, 4),
            TestCase(NUM_TILES - 2, 0, 3),
            TestCase(0, NUM_TILES - 2, 3),
            TestCase(NUM_TILES - 1, NUM_TILES - 2, 3),
            TestCase(NUM_TILES - 2, NUM_TILES - 1, 3),
        ];

        tests.iter().for_each(|t| {
            let grid = Grid::new();
            let result = grid.get_reachable_neighbour_positions(&Position {
                x: t.0,
                y: t.1,
            });
            assert_eq!(t.2, result.len());
        });
    }

    #[test]
    fn get_reachable_neighbour_tiles_with_walls() {
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);

        // place horizontal wall
        let mut g = Grid::new();
        let result = g.get_reachable_neighbour_positions(&Position::new(0, 0));
        assert_eq!(result.len(), 2);
        let b = Wall::new(Placement::new(0, 0), Orientation::Horizontal);
        assert!(g.place_wall(&p1, &p2, b));
        let result = g.get_reachable_neighbour_positions(&Position::new(0, 0));
        assert_eq!(1, result.len());
        assert_eq!(result[0].position, Position::new(1, 0));

        // place vertical wall
        let mut g = Grid::new();
        let b = Wall::new(Placement::new(0, 0), Orientation::Vertical);
        assert!(g.place_wall(&p1, &p2, b));
        let result = g.get_reachable_neighbour_positions(&Position::new(0, 0));
        assert_eq!(1, result.len());
        assert_eq!(Position::new(0, 1), result[0].position);

        // place vertical wall mid-grid
        let mut g = Grid::new();
        let b = Wall::new(Placement::new(4, 0), Orientation::Horizontal);
        assert!(g.place_wall(&p1, &p2, b));
        let result = g.get_reachable_neighbour_positions(&Position::new(4, 0));
        assert_eq!(2, result.len());
        // // only bottom and right
        assert_eq!(Position::new(3, 0), result[0].position);
        assert_eq!(Position::new(5, 0), result[1].position);

        // place vertical wall mid-grid
        let mut g = Grid::new();
        let b = Wall::new(Placement::new(0, 3), Orientation::Vertical);
        assert!(g.place_wall(&p1, &p2, b));
        let result = g.get_reachable_neighbour_positions(&Position::new(4, 0));
        assert_eq!(2, result.len());
        // only bottom and right
        assert_eq!(Position::new(4, 1), result[0].position);
        assert_eq!(Position::new(5, 0), result[1].position);
    }

    #[test]
    fn get_reachable_neighbour_tiles_with_walls_middle_of_the_board() {
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);
        let mut p3 = Player::new(String::new(), StartingPosition::Bottom);
        p3.position = Position { x: 4, y: 1 };
        let b1 = Wall::new(Placement::new(0, 3), Orientation::Vertical);
        let b2 = Wall::new(Placement::new(0, 4), Orientation::Vertical);

        // place vertical wall mid-grid
        let mut g = Grid::new();
        assert!(g.place_wall(&p1, &p2, b1));
        assert!(g.place_wall(&p1, &p2, b2));
        let result = g.get_reachable_neighbour_positions(&p1.position);
        assert_eq!(1, result.len());
        // only bottom and right
        assert_eq!(Position::new(4, 1), result[0].position);

        let mut g = Grid::new();
        assert!(g.place_wall(&p1, &p2, b1));
        assert!(g.place_wall(&p1, &p2, b2));
        let result = g.get_reachable_neighbour_positions(&p3.position);
        assert_eq!(2, result.len());
        assert_eq!(
            vec![Position::new(4, 0), Position::new(4, 2)],
            result.iter().map(|t| t.position).collect::<Vec<Position>>()
        );
    }

    #[test]
    fn get_reachable_neighbour_tiles_assert() {
        let g = Grid::new();
        let result = g.get_reachable_neighbour_positions(&Position::new(0, 0));
        assert_eq!(2, result.len());
        assert_eq!(0, result[0].position.x);
        assert_eq!(1, result[0].position.y);

        assert_eq!(1, result[1].position.x);
        assert_eq!(0, result[1].position.y);
    }

    /********************************************/
    //         can_player_reach_goal
    /********************************************/
    #[test]
    fn can_players_reach_end() {
        let mut g = Grid::new();
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);
        let wall = &Wall::new(Placement::new(0, 0), Orientation::Horizontal);
        assert!(g.place_wall(&p1, &p2, *wall));
        assert!(g.can_player_reach_goal(&p1));
        assert!(g.can_player_reach_goal(&p2));
    }

    #[test]
    fn test_can_player_reach_goal_normal() {
        struct TestCase(Vec<Wall>, Player, Player);

        let tests = [
            TestCase(
                vec![],
                Player::new(String::from("top"), StartingPosition::Top),
                Player::new(String::from("bottom"), StartingPosition::Bottom),
            ),
            TestCase(
                vec![
                    Wall::new(Placement::new(0, 3), Orientation::Horizontal),
                    Wall::new(Placement::new(2, 3), Orientation::Horizontal),
                    Wall::new(Placement::new(4, 3), Orientation::Horizontal),
                    Wall::new(Placement::new(6, 3), Orientation::Horizontal),
                    Wall::new(Placement::new(7, 5), Orientation::Horizontal),
                ],
                Player::new(String::from("top"), StartingPosition::Top),
                Player::new(String::from("bottom"), StartingPosition::Bottom),
            ),
            TestCase(
                vec![
                    Wall::new(Placement::new(0, 3), Orientation::Horizontal),
                    Wall::new(Placement::new(2, 3), Orientation::Horizontal),
                    Wall::new(Placement::new(4, 3), Orientation::Horizontal),
                    Wall::new(Placement::new(6, 3), Orientation::Horizontal),
                    Wall::new(Placement::new(7, 5), Orientation::Horizontal),
                    Wall::new(Placement::new(4, 6), Orientation::Vertical),
                ],
                Player {
                    position: Position::new(4, 4),
                    name: String::new(),
                    starting_position: StartingPosition::Top,
                    remaining_walls: NUM_WALLS,
                },
                Player {
                    position: Position::new(4, 3),
                    name: String::new(),
                    starting_position: StartingPosition::Bottom,
                    remaining_walls: NUM_WALLS,
                },
            ),
        ];

        tests.iter().for_each(|t| {
            let mut g = Grid::new();
            t.0.iter().for_each(|wall| {
                assert!(g.can_place_wall(&t.1, &t.2, *wall), "{:?}", wall);
                assert!(g.place_wall(&t.1, &t.2, *wall), "{:?}", wall);
            });

            assert!(g.can_player_reach_goal(&t.1), "player 1 {:?}", t.1);
            assert!(g.can_player_reach_goal(&t.2), "player 2 {:?}", t.2);
        });
    }

    #[test]
    fn test_can_player_reach_goal_disallowed_wall_formations_from_correct_position()
     {
        let walls = [
            Wall::new(Placement::new(0, 3), Orientation::Horizontal),
            Wall::new(Placement::new(2, 3), Orientation::Horizontal),
            Wall::new(Placement::new(4, 3), Orientation::Horizontal),
            Wall::new(Placement::new(6, 3), Orientation::Horizontal),
            Wall::new(Placement::new(7, 5), Orientation::Horizontal),
            Wall::new(Placement::new(4, 6), Orientation::Vertical),
        ];
        let mut g = Grid::new();
        walls.iter().for_each(|wall| match wall.orientation {
            Orientation::Horizontal => {
                g.horizontal_walls.push(*wall);
            }
            Orientation::Vertical => {
                g.vertical_walls.push(*wall);
            }
        });

        // test players can reach from end
        let mut p1 = Player::new(String::from("t"), StartingPosition::Top);
        p1.position = Position::new(4, 4);
        let mut p2 = Player::new(String::from("b"), StartingPosition::Bottom);
        p2.position = Position::new(4, 3);

        assert!(g.can_player_reach_goal(&p1));
        assert!(g.can_player_reach_goal(&p2));
    }

    #[test]
    fn test_can_place_wall_after_players_move() {
        let walls = [
            Wall::new(Placement::new(0, 3), Orientation::Horizontal),
            Wall::new(Placement::new(2, 3), Orientation::Horizontal),
            Wall::new(Placement::new(4, 3), Orientation::Horizontal),
            Wall::new(Placement::new(6, 3), Orientation::Horizontal),
            Wall::new(Placement::new(7, 5), Orientation::Horizontal),
        ];
        let mut g = Grid::new();
        let mut p1 = Player::new(String::from("t"), StartingPosition::Top);
        let mut p2 = Player::new(String::from("b"), StartingPosition::Bottom);

        assert!(walls.iter().all(|wall| g.place_wall(&p1, &p2, *wall)));

        let wall_dividing_board =
            Wall::new(Placement::new(4, 6), Orientation::Vertical);
        assert!(!g.place_wall(&p1, &p2, wall_dividing_board));
        assert!(g.can_player_reach_goal(&p1));
        assert!(g.can_player_reach_goal(&p2));

        p1.position = Position::new(4, 4);
        p2.position = Position::new(4, 3);
        assert!(g.place_wall(&p1, &p2, wall_dividing_board));
        assert!(g.can_player_reach_goal(&p1));
        assert!(g.can_player_reach_goal(&p2));
    }

    #[test]
    fn test_can_not_player_reach_goal_disallowed_wall_formations() {
        let walls = [
            Wall::new(Placement::new(0, 3), Orientation::Horizontal),
            Wall::new(Placement::new(2, 3), Orientation::Horizontal),
            Wall::new(Placement::new(4, 3), Orientation::Horizontal),
            Wall::new(Placement::new(6, 3), Orientation::Horizontal),
            Wall::new(Placement::new(7, 5), Orientation::Horizontal),
            Wall::new(Placement::new(4, 6), Orientation::Vertical),
        ];
        let mut g = Grid::new();
        walls.iter().for_each(|wall| match wall.orientation {
            Orientation::Horizontal => {
                g.horizontal_walls.push(*wall);
            }
            Orientation::Vertical => {
                g.vertical_walls.push(*wall);
            }
        });

        // test players can't reach from start
        let p1 = Player::new(String::from("t"), StartingPosition::Top);
        let p2 = Player::new(String::from("b"), StartingPosition::Bottom);

        let _neighbours = g.get_reachable_neighbour_positions(&p1.position);
        assert!(!g.can_player_reach_goal(&p1));
        assert!(!g.can_player_reach_goal(&p2));
    }

    /********************************************/
    //         place_wall
    /********************************************/
    #[test]
    fn can_place_weird_walls() {
        let b1 = Wall::new(Placement::new(0, 1), Orientation::Horizontal);
        let b2 = Wall::new(Placement::new(0, 1), Orientation::Vertical);
        let b3 = Wall::new(Placement::new(0, 0), Orientation::Vertical);
        let mut g = Grid::new();
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);

        assert!(!g.intersects_existing_walls(&b1));
        assert!(g.place_wall(&p1, &p2, b1));
        assert!(!g.intersects_existing_walls(&b2));
        assert!(g.place_wall(&p1, &p2, b2));
        assert!(!g.intersects_existing_walls(&b3));
        assert!(g.place_wall(&p1, &p2, b3));
    }

    /********************************************/
    //         does_intersect_existing_walls
    /********************************************/
    #[test]
    fn verify_does_intersect_existing_walls() {
        // setup
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);

        let mut g = Grid::new();
        let b1 = Wall::new(Placement::new(4, 3), Orientation::Horizontal);
        let b2 = Wall::new(Placement::new(3, 4), Orientation::Vertical);
        assert!(g.place_wall(&p1, &p2, b1));
        assert!(g.intersects_existing_walls(&b2));
        assert!(!g.place_wall(&p1, &p2, b2));

        let mut g = Grid::new();
        let b1 = Wall::new(Placement::new(3, 4), Orientation::Vertical);
        let b2 = Wall::new(Placement::new(4, 3), Orientation::Horizontal);
        assert!(g.place_wall(&p1, &p2, b1));
        assert!(g.intersects_existing_walls(&b2));
        assert!(!g.place_wall(&p1, &p2, b2));

        let mut g = Grid::new();
        let b1 = Wall::new(Placement::new(0, 0), Orientation::Horizontal);
        let b2 = Wall::new(Placement::new(0, 0), Orientation::Vertical);
        assert!(g.place_wall(&p1, &p2, b1));
        assert!(g.intersects_existing_walls(&b2));
        assert!(!g.place_wall(&p1, &p2, b2));
    }

    #[test]
    fn verify_walls_do_not_intersect() {
        let mut g = Grid::new();
        let p1 = &Player::new(String::new(), StartingPosition::Top);
        let p2 = &Player::new(String::new(), StartingPosition::Bottom);
        let b1 = &Wall {
            orientation: Orientation::Vertical,

            placement: Placement::new(7, 7),
        };
        let b2 = &Wall {
            orientation: Orientation::Horizontal,

            placement: Placement::new(5, 7),
        };
        let b3 = &Wall {
            orientation: Orientation::Horizontal,

            placement: Placement::new(5, 7),
        };
        let b4 = &Wall {
            orientation: Orientation::Horizontal,

            placement: Placement::new(5, 6),
        };
        assert!(g.place_wall(p1, p2, *b1));
        assert!(!g.intersects_existing_walls(b2));
        assert!(!g.intersects_existing_walls(b3));
        assert!(!g.intersects_existing_walls(b4));
    }

    #[test]
    fn verify_cant_place_crossing_walls() {
        struct TestCase(Wall, Wall);

        let tests = [
            TestCase(
                Wall {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(0, 0),
                },
                Wall {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(0, 0),
                },
            ),
            TestCase(
                Wall {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(1, 1),
                },
                Wall {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(1, 1),
                },
            ),
            TestCase(
                Wall {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(3, 3),
                },
                Wall {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(3, 3),
                },
            ),
        ];

        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);
        for t in tests.iter() {
            let mut grid = Grid::new();
            assert!(grid.place_wall(&p1, &p2, t.0));
            assert!(!grid.place_wall(&p1, &p2, t.1));
        }
    }

    #[test]
    fn verify_can_place_t_junctions() {
        struct TestCase(Wall, Wall);

        let tests = [
            TestCase(
                Wall {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(0, 0),
                },
                Wall {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(0, 1),
                },
            ),
            TestCase(
                Wall {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(0, 0),
                },
                Wall {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(1, 0),
                },
            ),
            TestCase(
                Wall {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(7, 7),
                },
                Wall {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(5, 6),
                },
            ),
            TestCase(
                Wall {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(7, 7),
                },
                Wall {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(5, 7),
                },
            ),
            TestCase(
                Wall {
                    orientation: Orientation::Vertical,

                    placement: Placement::new(7, 7),
                },
                Wall {
                    orientation: Orientation::Horizontal,

                    placement: Placement::new(7, 6),
                },
            ),
            TestCase(
                Wall::new(Placement::new(6, 3), Orientation::Horizontal),
                Wall::new(Placement::new(4, 6), Orientation::Vertical),
            ),
        ];

        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);
        for t in tests {
            let mut grid = Grid::new();
            assert!(grid.place_wall(&p1, &p2, t.0));
            assert!(
                grid.place_wall(&p1, &p2, t.1),
                "existing {:?} and new {:?} intersect",
                t.0,
                t.1,
            );
        }
    }

    #[test]
    fn test_can_place_wall() {
        let p1 = Player::new(String::new(), StartingPosition::Top);
        let p2 = Player::new(String::new(), StartingPosition::Bottom);
        let mut grid = Grid::new();

        assert!(grid.can_place_wall(
            &p1,
            &p2,
            Wall::new(Placement { tile: 4, lane: 4 }, Orientation::Horizontal)
        ));

        assert!(grid.can_place_wall(
            &p1,
            &p2,
            Wall::new(Placement { tile: 4, lane: 7 }, Orientation::Horizontal)
        ));

        assert!(!grid.can_place_wall(
            &p1,
            &p2,
            Wall::new(Placement { tile: 4, lane: 8 }, Orientation::Horizontal)
        ));

        assert!(!grid.can_place_wall(
            &p1,
            &p2,
            Wall::new(Placement { tile: 8, lane: 4 }, Orientation::Horizontal)
        ));
    }
}
