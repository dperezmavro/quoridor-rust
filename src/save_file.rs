use crate::core::player::Player;
use crate::core::wall::Wall;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SaveFile {
    pub p1_score: usize,
    pub p2_score: usize,
    pub total_rounds: usize,
    pub players: [Player; 2],
    pub walls: Vec<Wall>,
}

impl SaveFile {}
