use std::f32::consts::FRAC_PI_2;

use macroquad::{
    audio::{play_sound, play_sound_once, PlaySoundParams},
    prelude::{
        animation::{AnimatedSprite, Animation},
        collections::storage,
        scene::Node,
        *,
    },
};

use crate::{
    assets::Assets,
    level::{self, Level},
};

pub struct Crab {
    pub position: Vec2,
    speed: i8,
    rotation: f32,
    animation: AnimatedSprite,
    facing: bool,
    step: f32,
}

impl Crab {
    pub fn create() -> Self {
        let tiles = &storage::get::<Assets>().tilemaps[storage::get::<Level>().0];
        let (x, y) = tiles
            .tiles("crab", None)
            .find_map(|(x, y, tile)| tile.is_some().then_some((x, y)))
            .expect("crab not found");
        Self {
            speed: 0,
            position: Vec2::new(x as f32 + 0.5, y as f32 + 0.5),
            rotation: 0.,
            animation: AnimatedSprite::new(
                200,
                200,
                &[
                    Animation {
                        name: "idle".to_owned(),
                        row: 0,
                        frames: 20,
                        fps: 12,
                    },
                    Animation {
                        name: "walk".to_owned(),
                        row: 1,
                        frames: 3,
                        fps: 12,
                    },
                ],
                true,
            ),
            facing: false,
            step: 0.,
        }
    }
}
impl Node for Crab {
    fn ready(mut crab: scene::RefMut<Self>)
    where
        Self: Sized,
    {
        crab.animation.set_animation(0);
    }

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
        if crab.speed == 0 {
            crab.animation.set_animation(0);
        } else {
            crab.facing = crab.speed < 0;
            crab.animation.set_animation(1);
        }
        crab.position.x += crab.speed as f32 * CRAB_SPEED;
        // Gravity
        crab.position.y += 0.15;

        let tiles = &storage::get::<Assets>().tilemaps[storage::get::<Level>().0];

        // 0 1
        // 2 3
        let collide = |crab: &scene::RefMut<Crab>, name: &str| {
            [
                (
                    (crab.position.x - 0.5).floor() as u32,
                    (crab.position.y - 0.5).floor() as u32,
                ),
                (
                    (crab.position.x - 0.5).ceil() as u32,
                    (crab.position.y - 0.5).floor() as u32,
                ),
                (
                    (crab.position.x - 0.5).floor() as u32,
                    (crab.position.y - 0.5).ceil() as u32,
                ),
                (
                    (crab.position.x - 0.5).ceil() as u32,
                    (crab.position.y - 0.5).ceil() as u32,
                ),
            ]
            .into_iter()
            .enumerate()
            .filter_map(|(idx, (x, y))| tiles.get_tile(name, x, y).as_ref().map(|_| idx))
            .collect::<Vec<_>>()
        };
        let mut in_air = false;
        crab.rotation = match &collide(&crab, "main")[..] {
            [2, 3] | [2] | [3] => {
                crab.position.y = (crab.position.y + 0.5).floor() - 0.5;
                0.
            }
            [1, 2, 3] | [1, 3] | [1] => {
                crab.position.x = crab.position.x.ceil() - 0.5;
                crab.position.y -= 0.1;
                crab.position.y -= crab.speed as f32 * CRAB_SPEED;
                -FRAC_PI_2
            }
            [0, 2, 3] | [0, 2] | [0] => {
                crab.position.x = crab.position.x.floor() + 0.5;
                crab.position.y -= 0.1;
                crab.position.y += crab.speed as f32 * CRAB_SPEED;
                FRAC_PI_2
            }
            [] => {
                in_air = true;
                0.
            }
            collisions => unreachable!("{:?}", collisions),
        };
        // Collide with ceiling after climbing
        if [0, 1] == collide(&crab, "main")[..] {
            crab.position.y = crab.position.y.floor() + 0.5;
        }
        if !collide(&crab, "diamond").is_empty() {
            play_sound_once(storage::get::<Assets>().item_picked);
            storage::store(level::State::Complete);
        }
        crab.step = clamp(crab.step - 1. / 60., 0., 0.2);
        if crab.step == 0. && crab.speed != 0 && !in_air {
            crab.step = 0.16;
            play_sound(
                storage::get::<Assets>().step,
                PlaySoundParams {
                    looped: false,
                    volume: 0.2,
                },
            );
        }
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
    }

    fn draw(mut crab: scene::RefMut<Self>)
    where
        Self: Sized,
    {
        crab.animation.update();
        draw_texture_ex(
            storage::get::<Assets>().crab,
            crab.position.x - 0.5,
            crab.position.y - 0.5,
            WHITE,
            DrawTextureParams {
                source: Some(crab.animation.frame().source_rect),
                dest_size: Some(Vec2::new(1., 1.)),
                flip_x: crab.facing,
                rotation: crab.rotation,
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

const CRAB_SPEED: f32 = 0.75 / 60.;
