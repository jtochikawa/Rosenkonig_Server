extern crate game_server;

fn main() {
    use game_server::game;

    let mut _game: game::_Game = game::_Game::_init();

    _game.game_loop();
}