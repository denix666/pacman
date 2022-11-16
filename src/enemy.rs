use macroquad::prelude::*;
extern crate rand;
use rand::{Rng};

const ANIMATION_SPEED: i32 = 8;
pub const ENEMY_STEP_MOVE: f32 = 5.0;

pub enum EnemyDir {
    Up,
    Down,
    Left,
    Right,
}

pub struct Enemy {
    pub x: f32,
    pub y: f32, 
    pub destroyed: bool,
    pub rect: Rect,
    down_textures: Vec<Texture2D>,
    up_textures: Vec<Texture2D>,
    left_textures: Vec<Texture2D>,
    right_textures: Vec<Texture2D>,
    update_interval: i32,
    cur_frame: usize,
    pub dir: EnemyDir,
    pub possible_moves_list: Vec<String>,
}

impl Enemy {
    pub async fn new(x:f32, y:f32)  -> Self {
        let mut down_sprites:Vec<Texture2D> = Vec::new();
        let mut up_sprites:Vec<Texture2D> = Vec::new();
        let mut left_sprites:Vec<Texture2D> = Vec::new();
        let mut right_sprites:Vec<Texture2D> = Vec::new();
        
        let enemy_type: &str = match rand::thread_rng().gen_range(0..=3) { 
            0 => "red",
            1 => "blue",
            2 => "pinc",
            _ => "green",
        };

        for i in 0..=1 {
            let path = format!("assets/images/enemy/{}_down_{}.png",enemy_type, i);
            down_sprites.push(load_texture(&path).await.unwrap());
        }

        for i in 0..=1 {
            let path = format!("assets/images/enemy/{}_up_{}.png",enemy_type, i);
            up_sprites.push(load_texture(&path).await.unwrap());
        }

        for i in 0..=1 {
            let path = format!("assets/images/enemy/{}_left_{}.png",enemy_type, i);
            left_sprites.push(load_texture(&path).await.unwrap());
        }

        for i in 0..=1 {
            let path = format!("assets/images/enemy/{}_right_{}.png",enemy_type, i);
            right_sprites.push(load_texture(&path).await.unwrap());
        }

        let dir: EnemyDir = match rand::thread_rng().gen_range(0..=3) { 
            0 => EnemyDir::Down,
            1 => EnemyDir::Left,
            2 => EnemyDir::Right,
            _ => EnemyDir::Up,
        };
        
        Self {
            x,
            y,
            destroyed: false,
            down_textures: down_sprites,
            up_textures: up_sprites,
            left_textures: left_sprites,
            right_textures: right_sprites,
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            update_interval: 0,
            cur_frame: 0,
            dir,
            possible_moves_list: vec![],
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
            EnemyDir::Up => {
                draw_texture(self.up_textures[self.cur_frame], self.x, self.y, WHITE);
            },
            EnemyDir::Down => {
                draw_texture(self.down_textures[self.cur_frame], self.x, self.y, WHITE);
            },
            EnemyDir::Left => {
                draw_texture(self.left_textures[self.cur_frame], self.x, self.y, WHITE);
            },
            EnemyDir::Right => {
                draw_texture(self.right_textures[self.cur_frame], self.x, self.y, WHITE);
            },
        }

        // define rect
        self.rect.x = self.x;
        self.rect.y = self.y;
    }
}
