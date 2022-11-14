use macroquad::prelude::*;

const ANIMATION_SPEED: i32 = 8;
pub const STEP_MOVE: f32 = 5.0;

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player {
    pub x: f32,
    pub y: f32, 
    pub dir: Dir,
    pub requested_dir: Dir,
    up_textures: Vec<Texture2D>,
    down_textures: Vec<Texture2D>,
    left_textures: Vec<Texture2D>,
    right_textures: Vec<Texture2D>,
    update_interval: i32,
    cur_frame: usize,
    pub rect: Rect,
}

impl Player {
    pub async fn new()  -> Self {
        let mut up_sprites:Vec<Texture2D> = Vec::new();
        for i in 0..=2 {
            let path = format!("assets/images/player/u_{}.png", i);
            up_sprites.push(load_texture(&path).await.unwrap());
        }

        let mut down_sprites:Vec<Texture2D> = Vec::new();
        for i in 0..=2 {
            let path = format!("assets/images/player/d_{}.png", i);
            down_sprites.push(load_texture(&path).await.unwrap());
        }

        let mut left_sprites:Vec<Texture2D> = Vec::new();
        for i in 0..=2 {
            let path = format!("assets/images/player/l_{}.png", i);
            left_sprites.push(load_texture(&path).await.unwrap());
        }

        let mut right_sprites:Vec<Texture2D> = Vec::new();
        for i in 0..=2 {
            let path = format!("assets/images/player/r_{}.png", i);
            right_sprites.push(load_texture(&path).await.unwrap());
        }

        Self {
            x: 550.0,
            y: 650.0,
            dir: Dir::Left,
            requested_dir: Dir::Left,
            up_textures: up_sprites,
            down_textures: down_sprites,
            left_textures: left_sprites,
            right_textures: right_sprites,
            update_interval: 0,
            cur_frame: 0,
            rect: Rect::new(0.0, 0.0, 50.0,50.0),
        }
    }

    pub fn draw(&mut self) {
        self.update_interval += 1;
        if self.update_interval > ANIMATION_SPEED {
            self.update_interval = 0;
            self.cur_frame += 1;
            if self.cur_frame == self.up_textures.len() {
                self.cur_frame = 0;
            }
        }

        match self.dir {
            Dir::Up => {
                draw_texture(self.up_textures[self.cur_frame], self.x, self.y, WHITE);
            },
            Dir::Down => {
                draw_texture(self.down_textures[self.cur_frame], self.x, self.y, WHITE);
            },
            Dir::Left => {
                draw_texture(self.left_textures[self.cur_frame], self.x, self.y, WHITE);
            },
            Dir::Right => {
                draw_texture(self.right_textures[self.cur_frame], self.x, self.y, WHITE);
            },
        }

        // define rect
        self.rect.x = self.x;
        self.rect.y = self.y;
    }
}
