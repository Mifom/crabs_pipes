use macroquad::prelude::{collections::storage, *};

use crate::assets::Assets;

pub async fn loop_menu(last_level: usize) -> usize {
    let mut level = None;
    loop {
        if let Some(level) = level {
            return level;
        }
        clear_background(WHITE);
        let width = screen_width();
        let height = screen_height();
        let button_radius = height / 10.;
        let center = Vec2::new(width / 2., height / 2.);
        draw_text_ex(
            "Crab's pipes",
            center.x - 3. * button_radius,
            center.y - 3. * button_radius,
            TextParams {
                font: storage::get::<Assets>().font,
                font_size: button_radius.round() as u16,
                color: RED,
                ..Default::default()
            },
        );
        draw_texture_ex(
            storage::get::<Assets>().play,
            center.x - button_radius / 2.,
            center.y - button_radius / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(button_radius * 2., button_radius * 2.)),
                ..Default::default()
            },
        );
        if is_mouse_button_pressed(MouseButton::Left)
            && (Vec2::from(mouse_position()) - center).length() < button_radius
        {
            level = Some(last_level);
        }
        for touch in touches() {
            if (touch.position - center).length() < button_radius {
                level = Some(last_level);
            }
        }
        next_frame().await;
    }
}
