use macroquad::{prelude::*, audio::{Sound, load_sound}};

pub struct Resources {
    pub intro_texture: Texture2D,
    pub font: Font,
    pub bonus: Sound,
    pub coin: Sound,
    //pub beginning: Sound,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            intro_texture: load_texture("assets/images/intro.png").await.unwrap(),
            font: load_ttf_font("assets/fonts/game_font.ttf").await.unwrap(),
            bonus: load_sound("assets/sounds/bonus.ogg").await.unwrap(),
            coin: load_sound("assets/sounds/coin.ogg").await.unwrap(),
            //beginning: load_sound("assets/sounds/beginning.ogg").await.unwrap(),
        }
    }
}