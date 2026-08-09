use uuid::Uuid;
use crate::position::Position;

pub enum Colour {
    White,
    Black,
}

pub struct Player {
    pub colour: Colour,
    pub name: String,
    id: String,
    position: Position,
}

impl Player {
    pub fn new(name: String, colour: Colour, grid_size: u32) -> Self {
        let id = Uuid::new_v4().to_string();
        let p_x = match colour {
            Colour::White => 0,
            Colour::Black => grid_size,
        };
        Self {
            name,
            colour,
            id,
            position: Position::new(
                p_x,
                grid_size / 2,
            ),
        }
    }

    pub fn get_id(self) -> String {
        self.id
    }
}
