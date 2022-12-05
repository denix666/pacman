use macroquad::{prelude::*, audio::{Sound, load_sound}};

pub const RES_WIDTH: i32 = 1150;
pub const RES_HEIGHT: i32 = 750;
pub const STARTING_AMOUNT_OF_ENEMY: i32 = 2;

pub struct Resources {
    pub intro_texture: Texture2D,
    pub font: Font,
    pub bonus: Sound,
    pub coin: Sound,
    pub big_coin: Sound,
    pub eat_ghost: Sound,
    pub siren: Sound,
    pub die: Sound,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            intro_texture: load_texture("assets/images/intro.png").await.unwrap(),
            font: load_ttf_font("assets/fonts/game_font.ttf").await.unwrap(),
            bonus: load_sound("assets/sounds/bonus.ogg").await.unwrap(),
            coin: load_sound("assets/sounds/coin.ogg").await.unwrap(),
            big_coin: load_sound("assets/sounds/big_coin.ogg").await.unwrap(),
            eat_ghost: load_sound("assets/sounds/eat_ghost.ogg").await.unwrap(),
            siren: load_sound("assets/sounds/siren.ogg").await.unwrap(),
            die: load_sound("assets/sounds/die.ogg").await.unwrap(),
        }
    }
}