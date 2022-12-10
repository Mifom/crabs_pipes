use macroquad::prelude::{
    collections::storage,
    scene::{add_node, Node},
    *,
};
use macroquad_tiled::Map;

use crate::{
    assets::Assets,
    crab::{Crab, Item},
};

pub struct Level(pub usize);
pub enum State {
    Back,
    Running,
    Complete,
}

pub async fn loop_level(level: usize) -> bool {
    let (tiles, item_position) = &storage::get::<Assets>().tilemaps[level];
    storage::store(Level(level));
    storage::store(State::Running);
    add_node(Tiles::create(tiles));
    add_node(Crab::create());
    add_node(Item {
        position: *item_position,
    });
    let res = loop {
        clear_background(WHITE);
        next_frame().await;
        match *storage::get::<State>() {
            State::Back => break false,
            State::Running => {}
            State::Complete => break true,
        }
    };
    scene::clear();
    res
}
struct Tiles {
    rect: Rect,
}

impl Tiles {
    fn create(tiles: &Map) -> Self {
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
        tiles.0.draw_tiles("main", node.rect, None);
    }

    fn update(_node: scene::RefMut<Self>)
    where
        Self: Sized,
    {
        if is_key_pressed(KeyCode::Escape) {
            storage::store(State::Back);
        }
    }
}
