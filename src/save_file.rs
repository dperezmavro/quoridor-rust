use crate::core::player::Player;
use crate::core::wall::Wall;

use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Serialize, Debug)]
pub struct SaveFile {
    pub p1_score: usize,
    pub p2_score: usize,
    pub total_rounds: usize,
    pub players: [Player; 2],
    pub walls: Vec<Wall>,
}

impl SaveFile {
    pub fn new(
        p1_score: usize,
        p2_score: usize,
        total_rounds: usize,
        players: [Player; 2],
        walls: Vec<Wall>,
    ) -> SaveFile {
        SaveFile {
            walls,
            players,
            p1_score,
            p2_score,
            total_rounds,
        }
    }

    pub fn save_file(&self) {
        let json_str = serde_json::to_string(&self).unwrap();
        println!("{}", json_str);
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("/tmp/quoridor-rs.save.json")
            .unwrap();
        f.write_all(json_str.as_bytes()).unwrap();
    }
}
