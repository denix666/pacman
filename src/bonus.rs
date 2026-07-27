use macroquad::prelude::*;

use crate::resources::Resources;

pub struct Bonus {
    pub x: f32,
    pub y: f32,
    pub destroyed: bool,
    pub rect: Rect,
    pub bonus_started_at: f64,
    texture_idx: usize,
}

impl Bonus {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            destroyed: false,
            rect: Rect::new(0.0, 0.0, 30.0, 30.0),
            bonus_started_at: get_time(),
            texture_idx: ::rand::random_range(0..=3_usize),
        }
    }

    pub fn draw(&mut self, res: &Resources) {
        draw_texture(&res.bonus_textures[self.texture_idx], self.x, self.y, WHITE);

        self.rect.x = self.x + 15.0;
        self.rect.y = self.y + 15.0;
    }
}
