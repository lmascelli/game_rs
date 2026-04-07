mod game;
use game::Game;

fn main() {
    let mut config = game::AppConfig::new("config.ini")
        .expect("Failed to create the config manager");
    config.load();
    let mut game = Game::create("Basic rs platformer", config, "assets/map0.tmx")
        .expect("Failed to create game");
    if let Err(err) = game.run() {
        eprintln!("[ERROR]: {err:?}");
    }
}
