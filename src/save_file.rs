use crate::core::player::Player;
use crate::core::wall::Wall;

use serde::Serialize;
use vfs::{PhysicalFS, VfsPath};

const SAVE_FILE_NAME: &str = "quoridor-rs.save.json";

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
        let final_path = self.fs.root().join(SAVE_FILE_NAME).unwrap();
        println!("Saving to: {:?}", final_path.as_str());

        final_path
            .create_file()
            .unwrap()
            .write_all(json_str.as_bytes())
            .unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::player::StartingPosition;
    use vfs::MemoryFS;

    #[test]
    fn test_save_file() {
        let mut sf = SaveFile::new(
            1,
            2,
            3,
            [
                Player::new(String::from("p1111"), StartingPosition::Top),
                Player::new(String::from("p222"), StartingPosition::Bottom),
            ],
            vec![],
        );

        let mem_root: VfsPath = MemoryFS::new().into();
        sf.fs = mem_root;

        sf.save_file();
    }
}
