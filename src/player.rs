use uuid::Uuid;

pub enum Colour{
    White,
    Black,
}

pub struct Player {
    pub colour: Colour,
    pub name: String,
    id: String,
    pos_x: usize,
    pos_y: usize,
}

impl Player {
    pub fn new(
        name: String,
        colour: Colour,
        grid_size: usize,
    ) -> Self {
        let id = Uuid::new_v4().to_string();
        let p_x = match colour {
            Colour::White => 0,
            Colour::Black => grid_size,
        };
        Self{
           name,
           colour,
           id,
           pos_y: grid_size / 2,
           pos_x: p_x,
        }
    }

    pub fn get_id(self) -> String {
        self.id
    }

}
