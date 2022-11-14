use macroquad::prelude::*;

pub struct Coin {
    pub x: f32,
    pub y: f32, 
    pub destroyed: bool,
    texture: Texture2D,
    pub rect: Rect,
}

impl Coin {
    pub async fn new(x:f32, y:f32, brick_type: &str)  -> Self {
        let path = format!("assets/images/{}_coin.png",brick_type);
        Self {
            x,
            y,
            destroyed: false,
            texture: load_texture(&path).await.unwrap(),
            rect: Rect::new(0.0, 0.0, 30.0,30.0),
        }
    }

    pub fn draw(&mut self) {
        if !self.destroyed {
            draw_texture(self.texture, self.x, self.y, WHITE);

            // define rect
            self.rect.x = self.x + 10.0;
            self.rect.y = self.y + 10.0;
        }
    }
}
