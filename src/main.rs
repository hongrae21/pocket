mod game;

use game::init_game;

pub fn main() {
    let mut g = init_game();
    g.run();
}
