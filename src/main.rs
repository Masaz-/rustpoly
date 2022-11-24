mod tools;
mod game;

use game::Game;
fn main() {
    let mut rp = Game::new();
    rp.init();
}
