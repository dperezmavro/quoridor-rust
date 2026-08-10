use crate::block::Block;
use crate::position::Position;
use uuid::Uuid;

pub enum Colour {
    White,
    Black,
}

pub struct Player {
    pub colour: Colour,
    pub name: String,
    pub id: String,
    pub position: Position,
    pub blocks: [Block; 10],
}

impl Player {
    pub fn new(name: String, colour: Colour, grid_size: u32) -> Self {
        let id = Uuid::new_v4().to_string();
        let p_x = match colour {
            Colour::White => 0,
            Colour::Black => grid_size,
        };
        let blocks: [Block; 10] = core::array::from_fn(|i: usize| Block::new());
        Self {
            name,
            colour,
            id,
            position: Position::new(p_x, grid_size / 2),
            blocks,
        }
    }
}
