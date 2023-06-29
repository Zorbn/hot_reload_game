pub mod game;

#[no_mangle]
pub fn start(game: &mut game::Game) {
    game.start();
}

#[no_mangle]
pub fn update(game: &mut game::Game) -> bool {
    game.update()
}