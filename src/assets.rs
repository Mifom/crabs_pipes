use macroquad::prelude::*;
use macroquad_tiled as tiled;

pub struct Assets {
    pub crab: Texture2D,
    pub tilemaps: Vec<tiled::Map>,
}

impl Assets {
    pub async fn create() -> Self {
        let tiles_json = load_string("assets/level.json").await.unwrap();
        let tileset = load_texture("assets/tileset.png").await.unwrap();
        let basic = load_string("assets/basic.tsj").await.unwrap();
        let tiles = tiled::load_map(
            &tiles_json,
            &[("tileset.png", tileset)],
            &[("basic.tsj", &basic)],
        )
        .unwrap();
        Self {
            crab: load_texture("assets/crab.png").await.unwrap(),
            tilemaps: vec![tiles],
        }
    }
}
