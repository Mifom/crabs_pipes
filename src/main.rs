#![feature(is_some_with)]
#![feature(iterator_try_collect)]
use assets::{Assets, LEVEL_NUM};
use level::loop_level;
use macroquad::experimental::collections::storage;
use menu::loop_menu;

mod assets;
mod crab;
mod level;
mod menu;

enum State {
    Menu(usize),
    Level(usize),
}

#[macroquad::main("Crab's pipe")]
async fn main() {
    storage::store(Assets::create().await);
    let mut state = State::Menu(0);
    loop {
        match state {
            State::Menu(last_level) => {
                let level = loop_menu(last_level).await;
                state = State::Level(level);
            }
            State::Level(level) => {
                if loop_level(level).await {
                    if level < LEVEL_NUM - 1 {
                        state = State::Level(level + 1);
                    } else {
                        state = State::Menu(0);
                    }
                } else {
                    state = State::Menu(level);
                }
            }
        }
    }
}
