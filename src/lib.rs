#[macro_use]
mod browser;
mod engine;
mod game;
mod game_segments;
mod game_state;
mod segments;
mod sound;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::console;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console::log_1(&JsValue::from_str(
        "Hello world with web_sys::console::log1",
    ));
    log!("testing log! macro :). Hello world again!");

    browser::spawn_local(async move {
        engine::GameLoop::start(game::WalkTheDog::new())
            .await
            .expect("could not start game loop");
    });

    Ok(())
}
