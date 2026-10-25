use crate::core::player::Player;
use crate::core::wall::Wall;

use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;
use vfs::{PhysicalFS, VfsPath};

#[derive(Serialize, Debug)]
pub struct SaveFile {
    pub p1_score: usize,
    pub p2_score: usize,
    pub total_rounds: usize,
    pub players: [Player; 2],
    pub walls: Vec<Wall>,
    #[serde(skip_serializing)]
    fs: VfsPath,
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
            fs: PhysicalFS::new(std::env::temp_dir()).into(),
        }
    }

    pub fn save_file(&self) {
        let json_str = serde_json::to_string(&self).unwrap();
        println!("{}", json_str);
        // let final_path = self.fs.join("quoridor-rs.save.json").unwrap();
        // if final_path.exists().unwrap() {
        //     final_path
        //         .open_file()
        //         .unwrap()
        //         .write_all(json_str.as_bytes())
        //         .unwrap();
        // } else {
        //     final_path
        //         .create_file()
        //         .unwrap()
        //         .write_all(&json_str.as_bytes())
        //         .unwrap();
        // }
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("/tmp/quoridor-rs.save.json")
            .unwrap();
        f.write_all(json_str.as_bytes()).unwrap();
    }
}
