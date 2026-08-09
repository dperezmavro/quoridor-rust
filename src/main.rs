mod game;
mod grid;
mod player;
mod block;
mod position;

use game::Game;

fn main() {
    let _g = Game::new(String::from("dio"), String::from("ksenia"), 9);
}
