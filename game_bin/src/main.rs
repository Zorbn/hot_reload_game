use game_lib::game;

#[cfg(feature = "dynamic_reload")]
mod run_dynamic_reloading;
#[cfg(feature = "dynamic_reload")]
use run_dynamic_reloading::run;
#[cfg(not(feature = "dynamic_reload"))]
mod run;
#[cfg(not(feature = "dynamic_reload"))]
use run::run;

fn main() {
    let mut game = game::Game::default();
    run(&mut game);
}
