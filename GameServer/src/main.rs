extern crate GameServer;

fn main() {
    use GameServer::game;

    let mut _game: game::_Game = game::_Game::_init();

    _game.game_loop();
}