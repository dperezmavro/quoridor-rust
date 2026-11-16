use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// ID for player 1
const PLAYER_1_ID: u8 = 0;
/// ID for player 2
const PLAYER_2_ID: u8 = 1;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(into = "u8", try_from = "u8")]
pub enum PlayerId {
    One,
    Two,
}

impl PlayerId {
    pub fn player_from_id(id: u8) -> PlayerId {
        PlayerId::try_from(id)
            .unwrap_or_else(|_| panic!("Invalid player id: {}", id))
    }
}

impl From<PlayerId> for u8 {
    fn from(turn: PlayerId) -> Self {
        match turn {
            PlayerId::One => PLAYER_1_ID,
            PlayerId::Two => PLAYER_2_ID,
        }
    }
}

impl TryFrom<u8> for PlayerId {
    type Error = String;

    fn try_from(id: u8) -> Result<Self, Self::Error> {
        match id {
            PLAYER_1_ID => Ok(PlayerId::One),
            PLAYER_2_ID => Ok(PlayerId::Two),
            _ => Err(format!("Invalid player id: {}", id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_any() {
        assert_eq!(PlayerId::One, 0u8.try_into().unwrap());
        assert_eq!(PlayerId::Two, 1u8.try_into().unwrap());
    }
}
