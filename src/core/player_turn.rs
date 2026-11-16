use serde::{Deserialize, Serialize, Serializer};

/// ID for player 1
const PLAYER_1_ID: u8 = 0;
/// ID for player 2
const PLAYER_2_ID: u8 = 1;

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub enum PlayerTurn {
    One,
    Two,
}

impl PlayerTurn {
    pub fn player_from_id(id: u8) -> PlayerTurn {
        match id {
            PLAYER_1_ID => PlayerTurn::One,
            PLAYER_2_ID => PlayerTurn::Two,
            _ => panic!("Invalid player id: {}", id),
        }
    }

    // pub fn deserialize() -> u8 {
    //     0
    // }
}

impl Serialize for PlayerTurn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(match self {
            PlayerTurn::One => PLAYER_1_ID,
            PlayerTurn::Two => PLAYER_2_ID,
        })
    }
}

// impl<'de> Visitor<'de> for PlayerTurn {
//     type Value = PlayerTurn;
//
//     fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
//         formatter.write_str("0 or 1")
//     }
//
//     fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
//     where
//         E: Error,
//     {
//         match v {
//             0 => Ok(PlayerTurn::One),
//             1 => Ok(PlayerTurn::Two),
//             _ => Err(E::custom(format!("Invalid player id: {}", v))),
//         }
//     }
// }
//
// impl<'de> Deserialize<'de> for PlayerTurn {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         deserializer.deserialize_u8(PlayerTurn::deserialize)
//     }
// }
