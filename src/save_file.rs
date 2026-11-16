use crate::core::player::Player;
use crate::core::wall::Wall;
use std::cell::RefCell;
use std::rc::Rc;

use crate::core::game_rounds_state::GameRoundsState;
use crate::core::player_id::PlayerId;
use serde::{Deserialize, Serialize};
use vfs::{PhysicalFS, VfsPath};

/// Default save file name
const SAVE_FILE_NAME: &str = "quoridor-rs.save.json";

/// Save file contents
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SaveFile {
    pub p1_score: usize,
    pub p2_score: usize,
    pub total_rounds: usize,
    pub current_player_id: PlayerId,
    pub players: [Player; 2],
    pub walls: Vec<Wall>,
}

impl SaveFile {
    pub fn to_game_round_state(&self) -> GameRoundsState {
        GameRoundsState::from_state(
            self.total_rounds,
            self.p1_score,
            self.p2_score,
            self.current_player_id,
            self.walls.clone(),
            self.players.clone(),
        )
    }
}

/// Responsible for saving and loading games
pub struct SaveFileManager {
    pub fs: VfsPath,
}

// impl Drop for SaveFileManager {
//     fn drop(&mut self) {
//         println!("[+] Saving file to: TODO");
//     }
// }

impl SaveFileManager {
    pub fn new() -> Self {
        Self {
            fs: PhysicalFS::new(std::env::temp_dir()).into(),
        }
    }

    /// Saves to disk
    pub fn save_file(&self, grs: Rc<RefCell<GameRoundsState>>) -> String {
        let sf: SaveFile = SaveFile {
            p1_score: grs.borrow().p1_score,
            p2_score: grs.borrow().p2_score,
            total_rounds: grs.borrow().total_rounds,
            current_player_id: grs.borrow().game.get_player_turn(),
            players: [
                grs.borrow().game.p1.clone(),
                grs.borrow().game.p2.clone(),
            ],
            walls: grs.borrow().game.grid.walls(),
        };

        let json_str = serde_json::to_string(&sf).unwrap();
        let final_path = self.fs.join(SAVE_FILE_NAME).unwrap();

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
        println!("Loading file from: {:?}", final_path.as_str());

        let res = final_path.read_to_string().unwrap();
        serde_json::from_str::<SaveFile>(&res).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use vfs::MemoryFS;

    #[test]
    fn test_save_file() {
        let mut grs = GameRoundsState::new();
        grs.p1_score = 1;
        grs.p2_score = 2;
        grs.total_rounds = 3;

        let sfm = SaveFileManager {
            fs: MemoryFS::new().into(),
        };

        let grs_cc = Rc::new(RefCell::new(grs));

        let location = sfm.save_file(grs_cc);
        assert_eq!(format!("/{SAVE_FILE_NAME}"), location.as_str());
    }

    #[test]
    fn test_load_file() {
        let sf = SaveFile {
            p1_score: 1,
            p2_score: 2,
            current_player_id: PlayerId::One,
            total_rounds: 3,
            players: [
                Player::new(String::from("p1111"), PlayerId::One),
                Player::new(String::from("p222"), PlayerId::Two),
            ],
            walls: vec![],
        };

        let mut grs = GameRoundsState::new();
        grs.game.p1.name = String::from("p1111");
        grs.game.p2.name = String::from("p222");
        grs.p1_score = 1;
        grs.p2_score = 2;
        grs.total_rounds = 3;

        let sfm = SaveFileManager {
            fs: MemoryFS::new().into(),
        };

        let grs_cc = Rc::new(RefCell::new(grs));
        let location = sfm.save_file(grs_cc);
        let res = sfm.load_file(location);

        assert_eq!(sf, res);
    }

    #[test]
    fn test_to_game_round_state() {
        let sf = SaveFile {
            p1_score: 1,
            p2_score: 2,
            current_player_id: PlayerId::One,
            total_rounds: 3,
            players: [
                Player::new(String::from("p1111"), PlayerId::One),
                Player::new(String::from("p222"), PlayerId::Two),
            ],
            walls: vec![],
        };

        let mut grs = GameRoundsState::new();
        grs.game.p1.name = String::from("p1111");
        grs.game.p2.name = String::from("p222");
        grs.p1_score = 1;
        grs.p2_score = 2;
        grs.total_rounds = 3;

        assert_eq!(sf.to_game_round_state(), grs);
    }
}
