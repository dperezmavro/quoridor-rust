use crate::core::game::{Game, PLAYER_1_ID};
use crate::core::grid;
use crate::core::player::{Player, StartingPosition};
use crate::save_file::SaveFile;
use std::io::Write;

#[derive(Debug)]
/// Global game state. Keeps track of multiple games
/// and can save the game state.
pub struct GameRoundsState {
    pub total_rounds: usize,
    pub game: Game,
    pub p1_score: usize,
    pub p2_score: usize,
}

impl Drop for GameRoundsState {
    fn drop(&mut self) {
        println!("GameRoundsState drop!");
        self.save_state_to_file();
    }
}

impl GameRoundsState {
    /// Constructor
    pub fn new() -> Self {
        GameRoundsState {
            total_rounds: 0,
            p1_score: 0,
            p2_score: 0,
            game: new_game(),
        }
    }

    /// Saves to a file
    pub fn save_state_to_file(&self) {
        let sf = SaveFile::new(
            self.p1_score,
            self.p2_score,
            self.total_rounds,
            [self.game.p1.clone(), self.game.p2.clone()],
            self.game.grid.walls(),
        );

        sf.save_file();
    }

    /// Record a winning game for a player
    pub fn record_win(&mut self, winner_id: u8) {
        if winner_id == PLAYER_1_ID {
            self.p1_score += 1;
        } else {
            self.p2_score += 1;
        }

        self.total_rounds += 1;
    }

    /// Returns the games' current score
    pub fn scores(&self) -> (usize, usize) {
        (self.p1_score, self.p2_score)
    }

    /// Wrapper around the public function
    pub fn new_game(&mut self) {
        self.game = new_game();
    }
}

/// Creates a new game.
pub fn new_game() -> Game {
    let grid = grid::Grid::new();

    let p1 = Player::new(String::from("p1"), StartingPosition::Top);
    let p2 = Player::new(String::from("p2"), StartingPosition::Bottom);
    Game::new(grid, p1, p2)
}

#[cfg(test)]
mod test {
    use crate::core::game::{PLAYER_1_ID, PLAYER_2_ID};
    use crate::game_rounds_state::GameRoundsState;

    #[test]
    fn test_record_win() {
        let mut gg = GameRoundsState::new();
        gg.record_win(PLAYER_1_ID);

        assert_eq!(gg.total_rounds, 1);
        assert_eq!(gg.p1_score, 1);
        assert_eq!(gg.p2_score, 0);

        gg.record_win(PLAYER_2_ID);
        assert_eq!(gg.total_rounds, 2);
        assert_eq!(gg.p1_score, 1);
        assert_eq!(gg.p2_score, 1);
    }

    #[test]
    fn test_get_scores() {
        let mut gg = GameRoundsState::new();
        gg.record_win(PLAYER_1_ID);

        assert_eq!(gg.scores(), (1, 0));
        gg.record_win(PLAYER_2_ID);
        assert_eq!(gg.scores(), (1, 1));
    }
}
