use crate::core::game::{Game, PLAYER_1_ID};
use crate::core::grid;
use crate::core::player::{Player, StartingPosition};

#[derive(Debug)]
pub struct GameRoundsState {
    pub total_rounds: usize,
    pub game: Game,
    pub p1_score: usize,
    pub p2_score: usize,
}

impl Drop for GameRoundsState {
    fn drop(&mut self) {
        println!("GameRoundsState drop!");
    }
}

impl GameRoundsState {
    pub fn new() -> Self {
        GameRoundsState {
            total_rounds: 0,
            p1_score: 0,
            p2_score: 0,
            game: new_game(),
        }
    }

    pub fn record_win(&mut self, winner_id: u8) {
        if winner_id == PLAYER_1_ID {
            self.p1_score += 1;
        } else {
            self.p2_score += 1;
        }

        self.total_rounds += 1;

        println!(
            "Player {winner_id} wins this round! Scores: P1 {}, P2 {}",
            self.p1_score, self.p2_score
        );
    }

    pub fn scores(&self) -> (usize, usize) {
        (self.p1_score, self.p2_score)
    }

    pub fn new_game(&mut self) {
        self.game = new_game();
    }
}
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
