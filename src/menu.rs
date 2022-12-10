use macroquad::{prelude::*, ui::root_ui};

pub async fn loop_menu() -> usize {
    loop {
        clear_background(WHITE);
        root_ui().label(None, "Crab's Pipe");
        if root_ui().button(None, "Play") {
            return 0;
        }
        if root_ui().button(None, "Exit") {
            std::process::exit(0);
        }
        next_frame().await;
    }
}
