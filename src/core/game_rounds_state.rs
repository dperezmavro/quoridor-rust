use crate::core::game::Game;
use crate::core::grid;
use crate::core::player::{Player, StartingPosition};
use crate::core::player_turn::{PLAYER_1_ID, PlayerTurn};
use crate::core::wall::Wall;

#[derive(Debug)]
/// Global game state. Keeps track of multiple games
/// and can save the game state.
pub struct GameRoundsState {
    pub total_rounds: usize,
    pub game: Game,
    pub p1_score: usize,
    pub p2_score: usize,
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

    pub fn from_state(
        total_rounds: usize,
        p1_score: usize,
        p2_score: usize,
        current_player_id: u8,
        walls: Vec<Wall>,
        players: [Player; 2],
    ) -> Self {
        let game = Game::with_state(
            players[0].clone(),
            players[1].clone(),
            PlayerTurn::player_from_id(current_player_id),
            walls,
        );
        GameRoundsState {
            total_rounds,
            p1_score,
            p2_score,
            game,
        }
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
    use crate::core::game_rounds_state::GameRoundsState;
    use crate::core::player_turn::{PLAYER_1_ID, PLAYER_2_ID};

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
