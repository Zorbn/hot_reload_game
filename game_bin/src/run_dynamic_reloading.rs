use std::{sync, time};

use game_lib::game;

const DEBUG_SHARED_LIB_DIRECTORY: &str = "target/debug";
const RELEASE_SHARED_LIB_DIRECTORY: &str = "target/release";

type StartFn = extern "C" fn(&mut game::Game);
type UpdateFn = extern "C" fn(&mut game::Game) -> bool;

struct LibHandler {
    lib: Option<sync::Arc<dynamic_reload::Lib>>,
}

impl LibHandler {
    fn new() -> Self {
        Self { lib: None }
    }

    unsafe fn load_lib(&mut self, lib: &sync::Arc<dynamic_reload::Lib>) {
        self.lib = Some(lib.clone());
    }

    fn reload_callback(
        &mut self,
        state: dynamic_reload::UpdateState,
        lib: Option<&sync::Arc<dynamic_reload::Lib>>,
    ) {
        match state {
            dynamic_reload::UpdateState::Before => {
                self.lib = None;
            }
            dynamic_reload::UpdateState::After => {
                println!("Reloaded the game");
                unsafe {
                    self.load_lib(lib.unwrap());
                }
            }
            dynamic_reload::UpdateState::ReloadFailed(_) => println!("Failed to reload the game"),
        }
    }
}

pub fn run(game: &mut game::Game) {
    println!("Running with dynamic reloading");

    let shared_lib_directory = if cfg!(debug_assertions) {
        DEBUG_SHARED_LIB_DIRECTORY
    } else {
        RELEASE_SHARED_LIB_DIRECTORY
    };

    let mut reload_handler = dynamic_reload::DynamicReload::new(
        Some(vec![shared_lib_directory]),
        Some(shared_lib_directory),
        dynamic_reload::Search::Default,
        time::Duration::from_millis(250),
    );

    let mut lib_handler = LibHandler::new();

    unsafe {
        match reload_handler.add_library("game_lib", dynamic_reload::PlatformName::Yes) {
            Ok(lib) => lib_handler.load_lib(&lib),
            Err(_) => panic!("Failed to load game library"),
        }
    }

    if let Some(ref lib) = lib_handler.lib {
        unsafe {
            let start = lib
                .lib
                .get::<StartFn>(b"start\0")
                .expect("Failed to load game start function");
            start(game);
        }
    }

    let mut is_running = true;

    unsafe {
        while is_running {
            reload_handler.update(&LibHandler::reload_callback, &mut lib_handler);

            if let Some(ref lib) = lib_handler.lib {
                let update = lib
                    .lib
                    .get::<UpdateFn>(b"update\0")
                    .expect("Failed to load game update function");
                is_running = update(game);
            }
        }
    }
}
