#![feature(is_some_with)]
#![feature(iterator_try_collect)]
use assets::Assets;
use level::loop_level;
use macroquad::experimental::collections::storage;
use menu::loop_menu;

mod assets;
mod crab;
mod level;
mod menu;

enum State {
    Menu,
    Level(usize),
}

#[macroquad::main("Crab's pipe")]
async fn main() {
    storage::store(Assets::create().await);
    let mut state = State::Menu;
    loop {
        match state {
            State::Menu => {
                let level = loop_menu().await;
                state = State::Level(level);
            }
            State::Level(level) => {
                loop_level(level).await;
                state = State::Menu;
            }
        }
    }
}
