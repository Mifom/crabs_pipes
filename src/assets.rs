use macroquad::{
    audio::{self, Sound},
    prelude::*,
    ui::{root_ui, Skin},
};
use macroquad_tiled as tiled;

pub struct Assets {
    pub crab: Texture2D,
    pub diamond: Texture2D,
    pub tilemaps: Vec<tiled::Map>,
    pub ui: Ui,
    pub item_picked: Sound,
    pub step: Sound,
    pub piped: Sound,
    pub play: Texture2D,
    pub font: Font,
}

const TEXT_COLOR: Color = Color::new(0.8, 0.2, 0.2, 1.);
const BUTTON_COLOR: Color = Color::new(0.6, 0.6, 0.6, 1.);
const BUTTON_HOVERED_COLOR: Color = Color::new(0.4, 0.4, 0.4, 1.);

const TILESET: &[u8] = include_bytes!("../assets/tileset.png");
const BASIC: &str = include_str!("../assets/basic.tsj");
const CRAB: &[u8] = include_bytes!("../assets/crab.png");
const PLAY: &[u8] = include_bytes!("../assets/play.png");
const DIAMOND: &[u8] = include_bytes!("../assets/diamond.png");
const ITEM_PICKED: &[u8] = include_bytes!("../assets/item.ogg");
const STEP: &[u8] = include_bytes!("../assets/step.ogg");
const PIPED: &[u8] = include_bytes!("../assets/Piped.ogg");

const FONT: &[u8] = include_bytes!("../assets/font.ttf");

const TILEMAPS: [&str; 2] = [
    include_str!("../assets/level.json"),
    include_str!("../assets/level_2.json"),
];

pub const LEVEL_NUM: usize = TILEMAPS.len();

impl Assets {
    pub async fn create() -> Self {
        let crab = Texture2D::from_file_with_format(CRAB, None);
        let play = Texture2D::from_file_with_format(PLAY, None);
        let diamond = Texture2D::from_file_with_format(DIAMOND, None);

        let font = load_ttf_font_from_bytes(FONT).unwrap();

        let tileset = Texture2D::from_file_with_format(TILESET, None);
        let tilemaps = TILEMAPS
            .iter()
            .map(|tilemap| {
                tiled::load_map(
                    tilemap,
                    &[("tileset.png", tileset)],
                    &[("basic.tsj", BASIC)],
                )
                .unwrap()
            })
            .collect();

        let item_picked = audio::load_sound_from_bytes(ITEM_PICKED).await.unwrap();
        let step = audio::load_sound_from_bytes(STEP).await.unwrap();
        let piped = audio::load_sound_from_bytes(PIPED).await.unwrap();
        Self {
            crab,
            play,
            diamond,
            tilemaps,
            ui: Ui::create(),
            item_picked,
            step,
            piped,
            font,
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
