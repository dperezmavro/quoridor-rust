use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// ID for player 1
const PLAYER_1_ID: u8 = 0;
/// ID for player 2
const PLAYER_2_ID: u8 = 1;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(into = "u8", try_from = "u8")]
pub enum PlayerTurn {
    One,
    Two,
}

impl PlayerTurn {
    pub fn player_from_id(id: u8) -> PlayerTurn {
        PlayerTurn::try_from(id)
            .unwrap_or_else(|_| panic!("Invalid player id: {}", id))
    }
}

impl From<PlayerTurn> for u8 {
    fn from(turn: PlayerTurn) -> Self {
        match turn {
            PlayerTurn::One => PLAYER_1_ID,
            PlayerTurn::Two => PLAYER_2_ID,
        }
    }
}

impl TryFrom<u8> for PlayerTurn {
    type Error = String;

    fn try_from(id: u8) -> Result<Self, Self::Error> {
        match id {
            PLAYER_1_ID => Ok(PlayerTurn::One),
            PLAYER_2_ID => Ok(PlayerTurn::Two),
            _ => Err(format!("Invalid player id: {}", id)),
        }
    }
}
