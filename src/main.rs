mod block;
mod game;
mod grid;
mod player;
mod position;

use game::Game;

fn main() {
    let mut g = Game::new(String::from("dio"), String::from("ksenia"), 9);
    g.start();
}
