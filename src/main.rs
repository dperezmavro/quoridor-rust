mod game;
mod player;
mod grid;

use game::Game;

fn main() {
    let g = Game::new(
        String::from("dio"),
        String::from("ksenia"),
        9,
    );
}
