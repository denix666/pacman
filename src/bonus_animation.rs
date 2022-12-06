use macroquad::prelude::*;

const ANIMATION_SPEED: i32 = 2;

pub struct BonusAnimation {
    pub x: f32,
    pub y: f32, 
    update_interval: i32,
    cur_frame: usize,
    textures: Vec<Texture2D>,
    pub destroyed: bool,
}

impl BonusAnimation {
    pub async fn new(x: f32, y: f32)  -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();
        for i in 0..=14 {
            let path = format!("assets/images/bonus_animation/{}.png", i);
            sprites.push(load_texture(&path).await.unwrap());
        }

        Self {
            x,
            y,
            update_interval: 0,
            cur_frame: 0,
            textures: sprites,
            destroyed: false,
        }
    }

    pub fn draw(&mut self) {
        if !self.destroyed {
            draw_texture(self.textures[self.cur_frame], self.x, self.y, WHITE);
            self.update_interval += 1;
            if self.update_interval > ANIMATION_SPEED {
                self.update_interval = 0;
                self.cur_frame += 1;
                if self.cur_frame == self.textures.len() {
                    self.destroyed = true;
                }
            }
        }
    }
}
