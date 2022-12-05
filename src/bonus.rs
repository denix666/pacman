use macroquad::prelude::*;
extern crate rand;
use rand::{Rng};

pub struct Bonus {
    pub x: f32,
    pub y: f32, 
    pub destroyed: bool,
    texture: Texture2D,
    pub rect: Rect,
    pub bonus_started_at: f64,
}

impl Bonus {
    pub async fn new(x:f32, y:f32)  -> Self {
        let bonus_type: &str = match rand::thread_rng().gen_range(0..=3) { 
            0 => "apple",
            1 => "cake",
            2 => "cherry",
            _ => "strawberry",
        };

        let path = format!("assets/images/bonus/{}.png",bonus_type);
        Self {
            x,
            y,
            destroyed: false,
            texture: load_texture(&path).await.unwrap(),
            rect: Rect::new(0.0, 0.0, 30.0,30.0),
            bonus_started_at: get_time(),
        }
    }

    pub fn draw(&mut self) {
        draw_texture(self.texture, self.x, self.y, WHITE);

        // define rect
        self.rect.x = self.x + 15.0;
        self.rect.y = self.y + 15.0;
    }
}
