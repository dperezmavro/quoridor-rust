/// ID for player 1
pub const PLAYER_1_ID: u8 = 0;
/// ID for player 2
pub const PLAYER_2_ID: u8 = 1;

#[derive(Debug, PartialEq)]
pub enum PlayerTurn {
    Player1,
    Player2,
}

impl PlayerTurn {
    pub fn get_id(&self) -> u8 {
        match &self {
            PlayerTurn::Player1 => PLAYER_1_ID,
            PlayerTurn::Player2 => PLAYER_2_ID,
        }
    }

    pub fn player_from_id(id: u8) -> PlayerTurn {
        match id {
            PLAYER_1_ID => PlayerTurn::Player1,
            PLAYER_2_ID => PlayerTurn::Player2,
            _ => panic!("Invalid player id: {}", id),
        }
    }
}
