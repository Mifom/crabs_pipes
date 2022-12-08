use macroquad::prelude::*;
use macroquad_tiled as tiled;

struct Crab {
    position: Vec2,
    speed: i8,
}

struct World {
    crab: Crab,
    tiles: tiled::Map,
}

enum Controls {
    Left,
    Stay,
    Right,
}

#[macroquad::main("Crab's pipe")]
async fn main() {
    let tiles_json = load_string("assets/level.json").await.unwrap();
    let tileset = load_texture("assets/tileset.png").await.unwrap();
    let basic = load_string("assets/basic.tsj").await.unwrap();
    let tiles = tiled::load_map(
        &tiles_json,
        &[("tileset.png", tileset)],
        &[("basic.tsj", &basic)],
    )
    .unwrap();
    let mut world = World {
        crab: Crab {
            speed: 0,
            position: Vec2::new(
                (tiles.raw_tiled_map.width as f32) / 2.,
                tiles.raw_tiled_map.height as f32 - 1.5,
            ),
        },
        tiles,
    };
    loop {
        let height = screen_height();
        let width = screen_width();
        let scale = height / 10.;

        let controls = get_controls();
        update(&mut world, controls);
        draw(&world, width / scale, height / scale);
        next_frame().await;
    }
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

fn update(world: &mut World, controls: Controls) {
    world.crab.speed += match controls {
        Controls::Left => -1,
        Controls::Stay => -world.crab.speed.signum(),
        Controls::Right => 1,
    };
    world.crab.speed = clamp(world.crab.speed, -10, 10);
    world.crab.position.x += world.crab.speed as f32 * CRAB_SPEED;
    // Left
    let left_tile = world.tiles.get_tile(
        "main",
        (world.crab.position.x - 0.5).floor() as u32,
        world.crab.position.y as u32,
    );
    let right_tile = world.tiles.get_tile(
        "main",
        (world.crab.position.x + 0.5).floor() as u32,
        world.crab.position.y as u32,
    );
    if left_tile.is_some() {
        world.crab.position.x = world.crab.position.x.floor() + 0.5;
    } else if right_tile.is_some() {
        world.crab.position.x = world.crab.position.x.ceil() - 0.5;
    }
}

fn draw(world: &World, width: f32, height: f32) {
    clear_background(WHITE);
    world.tiles.draw_tiles(
        "main",
        Rect {
            x: 0.,
            y: 0.,
            w: (world.tiles.raw_tiled_map.width as f32),
            h: (world.tiles.raw_tiled_map.height as f32),
        },
        None,
    );
    draw_circle(world.crab.position.x, world.crab.position.y, 0.5, RED);
    let camera = Camera2D::from_display_rect(Rect {
        x: world.crab.position.x - width / 2.,
        y: (world.crab.position.y + 1.) - height,
        w: width,
        h: height,
    });
    set_camera(&camera);
}
