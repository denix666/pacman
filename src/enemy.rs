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

#[derive(PartialEq)]
pub enum EnemyMode {
    Normal,
    Scared,
    Eyes,
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
    scared_down_textures: Vec<Texture2D>,
    scared_up_textures: Vec<Texture2D>,
    scared_left_textures: Vec<Texture2D>,
    scared_right_textures: Vec<Texture2D>,
    eyes_down_texture: Texture2D,
    eyes_up_texture: Texture2D,
    eyes_left_texture: Texture2D,
    eyes_right_texture: Texture2D,
    update_interval: i32,
    cur_frame: usize,
    pub speed: f32,
    pub dir: EnemyDir,
    pub possible_moves_list: Vec<String>,
    pub enemy_mode: EnemyMode,
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

        let mut scared_down_sprites:Vec<Texture2D> = Vec::new();
        let mut scared_up_sprites:Vec<Texture2D> = Vec::new();
        let mut scared_left_sprites:Vec<Texture2D> = Vec::new();
        let mut scared_right_sprites:Vec<Texture2D> = Vec::new();

        for i in 0..=1 {
            let path = format!("assets/images/enemy/scared_down_{}.png", i);
            scared_down_sprites.push(load_texture(&path).await.unwrap());
        }

        for i in 0..=1 {
            let path = format!("assets/images/enemy/scared_up_{}.png", i);
            scared_up_sprites.push(load_texture(&path).await.unwrap());
        }

        for i in 0..=1 {
            let path = format!("assets/images/enemy/scared_left_{}.png", i);
            scared_left_sprites.push(load_texture(&path).await.unwrap());
        }

        for i in 0..=1 {
            let path = format!("assets/images/enemy/scared_right_{}.png", i);
            scared_right_sprites.push(load_texture(&path).await.unwrap());
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
            scared_down_textures: scared_down_sprites,
            scared_up_textures: scared_up_sprites,
            scared_left_textures: scared_left_sprites,
            scared_right_textures: scared_right_sprites,
            eyes_down_texture: load_texture("assets/images/eyes/down.png").await.unwrap(),
            eyes_up_texture: load_texture("assets/images/eyes/up.png").await.unwrap(),
            eyes_left_texture: load_texture("assets/images/eyes/left.png").await.unwrap(),
            eyes_right_texture: load_texture("assets/images/eyes/right.png").await.unwrap(),
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            update_interval: 0,
            cur_frame: 0,
            dir,
            possible_moves_list: vec![],
            enemy_mode: EnemyMode::Normal,
            speed: ENEMY_STEP_MOVE,
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
                match self.enemy_mode {
                    EnemyMode::Normal => {
                        draw_texture(self.up_textures[self.cur_frame], self.x, self.y, WHITE);
                    },
                    EnemyMode::Scared => {
                        draw_texture(self.scared_up_textures[self.cur_frame], self.x, self.y, WHITE);
                    },
                    EnemyMode::Eyes => {
                        draw_texture(self.eyes_up_texture, self.x, self.y, WHITE);
                    },
                };
            },
            EnemyDir::Down => {
                match self.enemy_mode {
                    EnemyMode::Normal => {
                        draw_texture(self.down_textures[self.cur_frame], self.x, self.y, WHITE);
                    },
                    EnemyMode::Scared => {
                        draw_texture(self.scared_down_textures[self.cur_frame], self.x, self.y, WHITE);
                    },
                    EnemyMode::Eyes => {
                        draw_texture(self.eyes_down_texture, self.x, self.y, WHITE);
                    },
                };
            },
            EnemyDir::Left => {
                match self.enemy_mode {
                    EnemyMode::Normal => {
                        draw_texture(self.left_textures[self.cur_frame], self.x, self.y, WHITE);
                    },
                    EnemyMode::Scared => {
                        draw_texture(self.scared_left_textures[self.cur_frame], self.x, self.y, WHITE);
                    },
                    EnemyMode::Eyes => {
                        draw_texture(self.eyes_left_texture, self.x, self.y, WHITE);
                    },
                };
            },
            EnemyDir::Right => {
                match self.enemy_mode {
                    EnemyMode::Normal => {
                        draw_texture(self.right_textures[self.cur_frame], self.x, self.y, WHITE);
                    },
                    EnemyMode::Scared => {
                        draw_texture(self.scared_right_textures[self.cur_frame], self.x, self.y, WHITE);
                    },
                    EnemyMode::Eyes => {
                        draw_texture(self.eyes_right_texture, self.x, self.y, WHITE);
                    },
                };
            },
        }

        // define rect
        self.rect.x = self.x;
        self.rect.y = self.y;
    }
}
