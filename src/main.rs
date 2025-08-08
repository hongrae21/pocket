mod game;
mod math;

use game::Game;

pub fn main() {
    let mut g = Game::new();
    g.run();
}
