mod block;
mod game;
mod grid;
mod player;
mod position;

use game::Game;

fn main() {
    let mut g = Game::new(String::from("d"), String::from("k"));
    g.start();
}
