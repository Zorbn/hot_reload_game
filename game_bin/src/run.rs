use game_lib::game;

pub fn run(game: &mut game::Game) {
    game_lib::start(game);

    while game_lib::update(game) {}
}