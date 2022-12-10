use macroquad::prelude::{
    collections::storage,
    scene::{add_node, Node},
    *,
};

use crate::{assets::Assets, crab::Crab};

pub struct Level(pub usize);
struct Stop(bool);

pub async fn loop_level(level: usize) {
    storage::store(Level(level));
    storage::store(Stop(false));
    add_node(Tiles::create());
    add_node(Crab::create());
    loop {
        clear_background(WHITE);
        next_frame().await;
        if storage::get::<Stop>().0 {
            break;
        }
    }
    scene::clear();
}
struct Tiles {
    rect: Rect,
}

impl Tiles {
    fn create() -> Self {
        let tiles = &storage::get::<Assets>().tilemaps[storage::get::<Level>().0];
        Self {
            rect: Rect {
                x: 0.,
                y: 0.,
                w: tiles.raw_tiled_map.width as f32,
                h: tiles.raw_tiled_map.height as f32,
            },
        }
    }
}

impl Node for Tiles {
    fn draw(node: scene::RefMut<Self>)
    where
        Self: Sized,
    {
        let tiles = &storage::get::<Assets>().tilemaps[storage::get::<Level>().0];
        tiles.draw_tiles("main", node.rect, None);
    }

    fn update(_node: scene::RefMut<Self>)
    where
        Self: Sized,
    {
        if is_key_pressed(KeyCode::Q) {
            storage::store(Stop(true));
        }
    }
}
