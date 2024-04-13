#[cfg(test)]
mod tests;

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod boid;
mod game;
mod palette;
mod point;
mod wasm4;
use game::Game;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
fn start() {
    // https://lospec.com/palette-list/water-shallows
    palette::set_palette([0x0b76bd, 0x0aa5bd, 0x011113, 0x0bbc81]);
}

#[no_mangle]
fn update() {
    GAME.lock().expect("game_state").update();
}
