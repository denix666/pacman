use macroquad::prelude::*;

use crate::resources::Resources;

pub struct Coin {
    pub x: f32,
    pub y: f32,
    pub destroyed: bool,
    pub rect: Rect,
    pub is_big: bool,
}

impl Coin {
    pub fn new(x: f32, y: f32, is_big: bool) -> Self {
        Self {
            x,
            y,
            destroyed: false,
            rect: Rect::new(0.0, 0.0, 30.0, 30.0),
            is_big,
        }
    }

    pub fn draw(&mut self, res: &Resources) {
        let texture = if self.is_big {
            &res.big_coin_texture
        } else {
            &res.small_coin_texture
        };
        draw_texture(texture, self.x, self.y, WHITE);

        self.rect.x = self.x + 15.0;
        self.rect.y = self.y + 15.0;
    }
}
