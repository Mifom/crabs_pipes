use macroquad::{
    prelude::*,
    ui::{root_ui, Skin},
};
use macroquad_tiled as tiled;

pub struct Assets {
    pub crab: Texture2D,
    pub tilemaps: Vec<tiled::Map>,
    pub ui: Ui,
}

const TILESET: &[u8] = include_bytes!("../assets/tileset.png");
const BASIC: &str = include_str!("../assets/basic.tsj");
const CRAB: &[u8] = include_bytes!("../assets/crab.png");
const TILEMAPS: [&str; 1] = [include_str!("../assets/level.json")];
const TEXT_COLOR: Color = Color::new(0.8, 0.2, 0.2, 1.);
const BUTTON_COLOR: Color = Color::new(0.6, 0.6, 0.6, 1.);
const BUTTON_HOVERED_COLOR: Color = Color::new(0.4, 0.4, 0.4, 1.);

impl Assets {
    pub fn create() -> Self {
        let crab = Texture2D::from_file_with_format(CRAB, None);
        let tileset = Texture2D::from_file_with_format(TILESET, None);
        let tilemaps = TILEMAPS
            .iter()
            .map(|tilemap| {
                tiled::load_map(
                    tilemap,
                    &[("tileset.png", tileset)],
                    &[("basic.tsj", BASIC)],
                )
            })
            .try_collect()
            .unwrap();

        Self {
            crab,
            tilemaps,
            ui: Ui::create(),
        }
    }
}

pub struct Ui {
    pub menu_skin: Skin,
}

impl Ui {
    fn create() -> Self {
        let menu_skin = {
            let label_style = root_ui()
                .style_builder()
                .font_size(40)
                .margin(RectOffset::new(20., 20., 10., 10.))
                .text_color(TEXT_COLOR)
                .build();
            let button_style = root_ui()
                .style_builder()
                .font_size(20)
                .color(BUTTON_COLOR)
                .color_hovered(BUTTON_HOVERED_COLOR)
                .text_color(TEXT_COLOR)
                .text_color_hovered(WHITE)
                .margin(RectOffset::new(50., 50., 5., 5.))
                .build();
            Skin {
                label_style,
                button_style,
                margin: 10.,
                ..root_ui().default_skin()
            }
        };
        Self { menu_skin }
    }
}
