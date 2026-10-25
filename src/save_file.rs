use crate::core::player::Player;
use crate::core::wall::Wall;

use serde::{Deserialize, Serialize};
use vfs::{PhysicalFS, VfsPath};

/// Default save file name
const SAVE_FILE_NAME: &str = "quoridor-rs.save.json";

/// Responsible for saving and loading games
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SaveFile {
    pub p1_score: usize,
    pub p2_score: usize,
    pub total_rounds: usize,
    pub players: [Player; 2],
    pub walls: Vec<Wall>,
}

/// Wraps the FS library and state object
pub struct SaveFileManager {
    sf: SaveFile,
    fs: VfsPath,
}

impl SaveFileManager {
    pub fn new(
        p1_score: usize,
        p2_score: usize,
        total_rounds: usize,
        players: [Player; 2],
        walls: Vec<Wall>,
    ) -> Self {
        Self {
            sf: SaveFile {
                walls,
                players,
                p1_score,
                p2_score,
                total_rounds,
            },
            fs: PhysicalFS::new(std::env::temp_dir()).into(),
        }
    }

    /// Saves to disk
    pub fn save_file(&self) -> String {
        let json_str = serde_json::to_string(&self.sf).unwrap();
        let final_path = self.fs.root().join(SAVE_FILE_NAME).unwrap();

        final_path
            .create_file()
            .unwrap()
            .write_all(json_str.as_bytes())
            .unwrap();
        String::from(final_path.as_str())
    }

    /// Reads a game
    pub fn load_file(&self, p: String) -> SaveFile {
        let final_path = self.fs.join(p).unwrap();
        println!("Loading from to: {:?}", final_path.as_str());

        let res = final_path.read_to_string().unwrap();
        serde_json::from_str::<SaveFile>(&res).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::player::StartingPosition;
    use vfs::MemoryFS;

    #[test]
    fn test_save_file() {
        let sf = SaveFile {
            p1_score: 1,
            p2_score: 2,
            total_rounds: 3,
            players: [
                Player::new(String::from("p1111"), StartingPosition::Top),
                Player::new(String::from("p222"), StartingPosition::Bottom),
            ],
            walls: vec![],
        };

        let sfm = SaveFileManager {
            sf,
            fs: MemoryFS::new().into(),
        };

        let location = sfm.save_file();
        assert_eq!(format!("/{SAVE_FILE_NAME}"), location.as_str());
    }

    #[test]
    fn test_load_file() {
        let sf = SaveFile {
            p1_score: 1,
            p2_score: 2,
            total_rounds: 3,
            players: [
                Player::new(String::from("p1111"), StartingPosition::Top),
                Player::new(String::from("p222"), StartingPosition::Bottom),
            ],
            walls: vec![],
        };

        let sfm = SaveFileManager {
            sf: sf.clone(),
            fs: MemoryFS::new().into(),
        };

        let location = sfm.save_file();

        let res = sfm.load_file(location);

        assert_eq!(sf, res);
    }
}
