mod game;
use game::Game;

fn main() {
    let mut game = Game::create("Basic rs platformer", 800, 600, "assets/map0.tmx")
        .expect("Failed to create game");
    if let Err(err) = game.run() {
        eprintln!("[ERROR]: {err:?}");
    }
}
