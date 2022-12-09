use macroquad::prelude::{collections::storage, scene::Node, *};

use crate::assets::Assets;

pub struct Crab {
    pub position: Vec2,
    speed: i8,
}

impl Crab {
    pub fn create() -> Self {
        let tiles = &storage::get::<Assets>().tiles;
        Self {
            speed: 0,
            position: Vec2::new(
                (tiles.raw_tiled_map.width as f32) / 2.,
                tiles.raw_tiled_map.height as f32 - 1.5,
            ),
        }
    }
}
impl Node for Crab {
    fn fixed_update(mut crab: scene::RefMut<Self>)
    where
        Self: Sized,
    {
        crab.speed += match get_controls() {
            Controls::Left => -1,
            Controls::Stay => -crab.speed.signum(),
            Controls::Right => 1,
        };
        crab.speed = clamp(crab.speed, -10, 10);
        crab.position.x += crab.speed as f32 * CRAB_SPEED;

        let tiles = &storage::get::<Assets>().tiles;
        let left_tile = tiles.get_tile(
            "main",
            (crab.position.x - 0.5).floor() as u32,
            crab.position.y as u32,
        );
        let right_tile = tiles.get_tile(
            "main",
            (crab.position.x + 0.5).floor() as u32,
            crab.position.y as u32,
        );
        if left_tile.is_some() {
            crab.position.x = crab.position.x.floor() + 0.5;
        } else if right_tile.is_some() {
            crab.position.x = crab.position.x.ceil() - 0.5;
        }
    }

    fn draw(crab: scene::RefMut<Self>)
    where
        Self: Sized,
    {
        // Set proper camera
        let height = screen_height();
        let width = screen_width();
        let scale = height / 5.;
        let width = width / scale;
        let height = height / scale;
        let camera = Camera2D::from_display_rect(Rect {
            x: crab.position.x - width / 2.,
            y: (crab.position.y + 1.) - height,
            w: width,
            h: height,
        });
        scene::set_camera(0, Some(camera));

        draw_texture_ex(
            storage::get::<Assets>().crab,
            crab.position.x - 0.5,
            crab.position.y - 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(1., 1.)),
                flip_x: crab.speed < 0,
                ..Default::default()
            },
        );
    }
}
enum Controls {
    Left,
    Stay,
    Right,
}

fn get_controls() -> Controls {
    let mut direction = 0i8;
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        direction -= 1;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        direction += 1;
    }
    if is_mouse_button_down(MouseButton::Left) {
        let position = mouse_position_local();
        if position.x < 0. {
            direction -= 1;
        } else if position.x > 0. {
            direction += 1;
        }
    }
    for touch in touches_local() {
        if touch.position.x < 0. {
            direction -= 1;
        } else if touch.position.x > 0. {
            direction += 1;
        }
    }
    match direction.cmp(&0) {
        std::cmp::Ordering::Less => Controls::Left,
        std::cmp::Ordering::Equal => Controls::Stay,
        std::cmp::Ordering::Greater => Controls::Right,
    }
}

const CRAB_SPEED: f32 = 1. / 60.;
