use uuid::Uuid;

use crate::player::{Colour, Player};

use crate::grid::{Grid, Tile, TileState, TileType};

pub struct Game {
    id: String,
    pub p1: Player,
    pub p2: Player,
    pub g: Grid,
    // TODO dio : other stuff like date started etc
}

impl Game {
    pub fn new(n1: String, n2: String, size: u32) -> Self {
        assert!(size % 2 != 0);
        let id = Uuid::new_v4().to_string();
        let p1 = Player::new(n1, Colour::White, size);
        let p2 = Player::new(n2, Colour::Black, size);
        Self {
            g: Grid::new(size, p1.name.clone(), p2.name.clone()),
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
        for row in self.g.tiles.iter() {
            print!("|");
            for tile in row.clone() {
                match tile.tile_type {
                    TileType::BlockPosition => {
                        print!("b")
                    }
                    TileType::BlockNode => {
                        print!("*");
                    }
                    TileType::Playable => match tile.state {
                        TileState::Free => {
                            print!(" ")
                        }
                        TileState::PlayerOccupied { player_id } => {
                            print!("P");
                        }
                        TileState::BlockOccupied { player_id } => {
                            print!("B");
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
    #[should_panic]
    fn game_new_even_size() {
        let g = Game::new(String::from("a"), String::from("a"), 4);
    }

    #[test]
    fn simple_game() {
        let g = Game::new(String::from("a"), String::from("a"), 5);
        assert_eq!(g.g.size, 5);
        assert_eq!(g.p1.position.y, 0);
        assert_eq!(g.p1.position.x, 2);

        assert_eq!(g.p2.position.y, 4);
        assert_eq!(g.p2.position.x, 2);
    }
}
