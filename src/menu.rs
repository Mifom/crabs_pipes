use macroquad::{
    prelude::{collections::storage, *},
    ui::{hash, root_ui},
};

use crate::assets::Assets;

const WINDOW_WIDTH: f32 = 300.;
const WINDOW_HEIGHT: f32 = 250.;

pub async fn loop_menu() -> usize {
    let mut level = None;
    loop {
        if let Some(level) = level {
            return level;
        }
        clear_background(WHITE);
        root_ui().push_skin(&storage::get::<Assets>().ui.menu_skin);
        root_ui().window(
            hash!(),
            Vec2::new(
                screen_width() / 2. - WINDOW_WIDTH / 2.,
                screen_height() / 2. - WINDOW_HEIGHT / 2.,
            ),
            Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            |ui| {
                ui.label(None, "Crab's Pipe");
                ui.separator();
                if ui.button(None, "  Play  ") {
                    level = Some(0);
                }
                if ui.button(None, "  Exit  ") {
                    std::process::exit(0);
                }
            },
        );
        root_ui().pop_skin();
        next_frame().await;
    }
}
