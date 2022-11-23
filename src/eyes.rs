use macroquad::prelude::*;
extern crate rand;
use rand::seq::SliceRandom;

pub const STEP_MOVE: f32 = 10.0;

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub struct Eyes {
    pub x: f32,
    pub y: f32, 
    pub destroyed: bool,
    down_texture: Texture2D,
    up_texture: Texture2D,
    left_texture: Texture2D,
    right_texture: Texture2D,
    pub dir: Dir,
    pub possible_moves_list: Vec<String>,
}

impl Eyes {
    pub async fn new(x:f32, y:f32)  -> Self {
        Self {
            x,
            y,
            destroyed: false,
            down_texture: load_texture("assets/images/eyes/down.png").await.unwrap(),
            up_texture: load_texture("assets/images/eyes/up.png").await.unwrap(),
            left_texture: load_texture("assets/images/eyes/left.png").await.unwrap(),
            right_texture: load_texture("assets/images/eyes/right.png").await.unwrap(),
            dir: Dir::Left,
            possible_moves_list: vec![],
        }
    }

    pub fn update(&mut self, points: &Vec<crate::points::Point>) {
        self.possible_moves_list.clear();

        match self.dir {
            Dir::Up => {
                let check_x: u32 = (self.x / 50.0) as u32;
                let check_y: u32 = ((self.y - 5.0) / 50.0) as u32;
                if crate::get_val(check_x, check_y, &points) != "X" {
                    self.y -= STEP_MOVE;
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
            Dir::Down => {
                let check_x: u32 = (self.x / 50.0) as u32;
                let check_y: u32 = (self.y / 50.0) as u32;
                if crate::get_val(check_x, check_y + 1, &points) != "X" {
                    self.y += STEP_MOVE;
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
            Dir::Left => {
                let check_x: u32 = ((self.x - 5.0) / 50.0) as u32;
                let check_y: u32 = (self.y / 50.0) as u32;
                if crate::get_val(check_x, check_y, &points) != "X" {
                    self.x -= STEP_MOVE;
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
            Dir::Right => {
                let check_x: u32 = (self.x / 50.0) as u32;
                let check_y: u32 = (self.y / 50.0) as u32;
                if crate::get_val(check_x + 1, check_y, &points) != "X" {
                    self.x += STEP_MOVE;
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
                    self.dir = Dir::Up;
                },
                "down" => {
                    self.dir = Dir::Down;
                },
                "left" => {
                    self.dir = Dir::Left;
                },
                "right" => {
                    self.dir = Dir::Right;
                },
                _ => {
                    panic!("unknown dir");
                }
            };
        }
    }

    pub fn draw(&mut self) {
        match self.dir {
            Dir::Up => {
                draw_texture(self.up_texture, self.x, self.y, WHITE);
            },
            Dir::Down => {
                draw_texture(self.down_texture, self.x, self.y, WHITE);
            },
            Dir::Left => {
                draw_texture(self.left_texture, self.x, self.y, WHITE);
            },
            Dir::Right => {
                draw_texture(self.right_texture, self.x, self.y, WHITE);
            },
        }
    }
}
