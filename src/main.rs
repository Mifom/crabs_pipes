#![feature(is_some_with)]
use assets::Assets;
use crab::Crab;
use macroquad::experimental::collections::storage;
use macroquad::prelude::scene::{self, add_node, Node};
use macroquad::prelude::*;

mod assets;
mod crab;

#[macroquad::main("Crab's pipe")]
async fn main() {
    storage::store(Assets::create().await);
    add_node(Tiles::create());
    add_node(Crab::create());
    loop {
        clear_background(WHITE);
        next_frame().await;
    }
}

struct Tiles {
    rect: Rect,
}

impl Tiles {
    fn create() -> Self {
        let tiles = &storage::get::<Assets>().tiles;
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
        let tiles = &storage::get::<Assets>().tiles;
        tiles.draw_tiles("main", node.rect, None);
    }
}
