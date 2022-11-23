use macroquad::prelude::*;
extern crate rand;
use rand::{Rng};
use rand::seq::SliceRandom;

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
    scared_down_textures: Vec<Texture2D>,
    scared_up_textures: Vec<Texture2D>,
    scared_left_textures: Vec<Texture2D>,
    scared_right_textures: Vec<Texture2D>,
    update_interval: i32,
    pub scared_mode: bool,
    cur_frame: usize,
    pub speed: f32,
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
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            update_interval: 0,
            cur_frame: 0,
            scared_mode: false,
            dir,
            possible_moves_list: vec![],
            speed: ENEMY_STEP_MOVE,
        }
    }

    pub fn update(&mut self, points: &Vec<crate::points::Point>) {
        self.possible_moves_list.clear();

        match self.dir {
            EnemyDir::Up => {
                let check_x: u32 = (self.x / 50.0) as u32;
                let check_y: u32 = ((self.y - 5.0) / 50.0) as u32;
                if crate::get_val(check_x, check_y, &points) != "X" {
                    self.y -= self.speed;
                }
                
                if self.y % 50.0 == 0.0 {
                    let check_x: u32 = ((self.x - 5.0) / 50.0) as u32;
                    let check_y: u32 = (self.y / 50.0) as u32;
                    if crate::get_val(check_x, check_y, &points) != "X" {
                        self.possible_moves_list.push("left".to_string());
                    }

                    let check_x: u32 = (self.x / 50.0) as u32;
                    let check_y: u32 = (self.y / 50.0) as u32;
                    if crate::get_val(check_x + 1, check_y, &points) != "X" {
                        self.possible_moves_list.push("right".to_string());
                    }
                }
            },
            EnemyDir::Down => {
                let check_x: u32 = (self.x / 50.0) as u32;
                let check_y: u32 = (self.y / 50.0) as u32;
                if crate::get_val(check_x, check_y + 1, &points) != "X" {
                    self.y += self.speed;
                }
                
                if self.y % 50.0 == 0.0 {
                    let check_x: u32 = ((self.x - 5.0) / 50.0) as u32;
                    let check_y: u32 = (self.y / 50.0) as u32;
                    if crate::get_val(check_x, check_y, &points) != "X" {
                        self.possible_moves_list.push("left".to_string());
                    }

                    let check_x: u32 = (self.x / 50.0) as u32;
                    let check_y: u32 = (self.y / 50.0) as u32;
                    if crate::get_val(check_x + 1, check_y, &points) != "X" {
                        self.possible_moves_list.push("right".to_string());
                    }
                }
            },
            EnemyDir::Left => {
                let check_x: u32 = ((self.x - 5.0) / 50.0) as u32;
                let check_y: u32 = (self.y / 50.0) as u32;
                if crate::get_val(check_x, check_y, &points) != "X" {
                    self.x -= self.speed;
                }
                
                if self.x % 50.0 == 0.0 {
                    let check_x: u32 = (self.x / 50.0) as u32;
                    let check_y: u32 = ((self.y - 5.0) / 50.0) as u32;
                    if crate::get_val(check_x, check_y, &points) != "X" {
                        self.possible_moves_list.push("up".to_string());
                    }

                    let check_x: u32 = (self.x / 50.0) as u32;
                    let check_y: u32 = (self.y / 50.0) as u32;
                    if crate::get_val(check_x, check_y + 1, &points) != "X" {
                        self.possible_moves_list.push("down".to_string());
                    }
                }
            },
            EnemyDir::Right => {
                let check_x: u32 = (self.x / 50.0) as u32;
                let check_y: u32 = (self.y / 50.0) as u32;
                if crate::get_val(check_x + 1, check_y, &points) != "X" {
                    self.x += self.speed;
                }
                
                if self.x % 50.0 == 0.0 {
                    let check_x: u32 = (self.x / 50.0) as u32;
                    let check_y: u32 = ((self.y - 5.0) / 50.0) as u32;
                    if crate::get_val(check_x, check_y, &points) != "X" {
                        self.possible_moves_list.push("up".to_string());
                    }

                    let check_x: u32 = (self.x / 50.0) as u32;
                    let check_y: u32 = (self.y / 50.0) as u32;
                    if crate::get_val(check_x, check_y + 1, &points) != "X" {
                        self.possible_moves_list.push("down".to_string());
                    }
                }
            },
        }

        if self.possible_moves_list.len() > 0 {
            match self.possible_moves_list.choose(&mut rand::thread_rng()).unwrap().as_str() {
                "up" => {
                    self.dir = EnemyDir::Up;
                },
                "down" => {
                    self.dir = EnemyDir::Down;
                },
                "left" => {
                    self.dir = EnemyDir::Left;
                },
                "right" => {
                    self.dir = EnemyDir::Right;
                },
                _ => {
                    panic!("unknown dir");
                }
            };
        }

        // define rect
        self.rect.x = self.x;
        self.rect.y = self.y;
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
                if !self.scared_mode {
                    draw_texture(self.up_textures[self.cur_frame], self.x, self.y, WHITE);
                } else {
                    draw_texture(self.scared_up_textures[self.cur_frame], self.x, self.y, WHITE);
                }
            },
            EnemyDir::Down => {
                if !self.scared_mode {
                    draw_texture(self.down_textures[self.cur_frame], self.x, self.y, WHITE);
                } else {
                    draw_texture(self.scared_down_textures[self.cur_frame], self.x, self.y, WHITE);
                }
            },
            EnemyDir::Left => {
                if !self.scared_mode {
                    draw_texture(self.left_textures[self.cur_frame], self.x, self.y, WHITE);
                } else {
                    draw_texture(self.scared_left_textures[self.cur_frame], self.x, self.y, WHITE);
                }
            },
            EnemyDir::Right => {
                if !self.scared_mode {
                    draw_texture(self.right_textures[self.cur_frame], self.x, self.y, WHITE);
                } else {
                    draw_texture(self.scared_right_textures[self.cur_frame], self.x, self.y, WHITE);
                }
            },
        }
    }
}
