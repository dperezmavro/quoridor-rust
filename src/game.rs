use uuid::Uuid;

use crate::player::{Colour, Player};

use crate::grid::{GRID_SIZE, Grid, Tile, TileState, TileType};

pub struct Game {
    id: String,
    pub p1: Player,
    pub p2: Player,
    pub g: Grid,
    // TODO dio : other stuff like date started etc
}

impl Game {
    pub fn new(n1: String, n2: String) -> Self {
        let id = Uuid::new_v4().to_string();
        let p1 = Player::new(n1, Colour::White, GRID_SIZE);
        let p2 = Player::new(n2, Colour::Black, GRID_SIZE);
        let g = Grid::new(p1.name.clone(), p2.name.clone());
        Self {
            g,
            p1: p1,
            p2: p2,
            id,
        }
    }

    pub fn start(&mut self) {
        self.render();
    }

    pub fn render(&self) {
        println!("_________________________________");
        for row in self.g.player_tiles.iter() {
            print!("|");
            for tile in row.clone() {
                match tile.tile_type {
                    TileType::BlockPosition => {
                        print!("b")
                    }
                    TileType::Playable => match tile.state {
                        TileState::Free => {
                            print!(" ")
                        }
                        TileState::Occupied => {
                            print!("P");
                        }
                    },
                }
            }
            println!("|")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_game() {
        let g = Game::new(String::from("a"), String::from("a"));
        assert_eq!(g.p1.position.y, 0);
        assert_eq!(g.p1.position.x, GRID_SIZE / 2);

        assert_eq!(g.p2.position.y, GRID_SIZE - 1);
        assert_eq!(g.p2.position.x, GRID_SIZE / 2);
    }
}
