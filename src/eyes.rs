use macroquad::prelude::*;
use ::rand::seq::IndexedRandom;

use crate::resources::Resources;

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
    pub dir: Dir,
    pub inside_spawn: bool,
    possible_moves_list: Vec<String>,
}

impl Eyes {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            destroyed: false,
            dir: Dir::Left,
            possible_moves_list: vec![],
            inside_spawn: false,
        }
    }

    pub fn update(&mut self, points: &[crate::points::Point]) {
        self.possible_moves_list.clear();

        if crate::levels::get_val((self.x / 50.0) as i32, (self.y / 50.0) as i32, points) == "s" {
            self.inside_spawn = true;
        }

        match self.dir {
            Dir::Up => {
                let check_x = (self.x / 50.0) as i32;
                let check_y = ((self.y - 5.0) / 50.0) as i32;
                if crate::levels::get_val(check_x, check_y, points) != "#" {
                    self.y -= STEP_MOVE;
                }

                if self.y % 50.0 == 0.0 {
                    let check_x = ((self.x - 5.0) / 50.0) as i32;
                    let check_y = (self.y / 50.0) as i32;
                    if crate::levels::get_val(check_x, check_y, points) != "#" {
                        self.possible_moves_list.push("left".to_string());
                    }

                    let check_x = (self.x / 50.0) as i32;
                    if crate::levels::get_val(check_x + 1, check_y, points) != "#" {
                        self.possible_moves_list.push("right".to_string());
                    }
                }
            },
            Dir::Down => {
                let check_x = (self.x / 50.0) as i32;
                let check_y = (self.y / 50.0) as i32;
                if crate::levels::get_val(check_x, check_y + 1, points) != "#" {
                    self.y += STEP_MOVE;
                }

                if self.y % 50.0 == 0.0 {
                    let check_x = ((self.x - 5.0) / 50.0) as i32;
                    let check_y = (self.y / 50.0) as i32;
                    if crate::levels::get_val(check_x, check_y, points) != "#" {
                        self.possible_moves_list.push("left".to_string());
                    }

                    let check_x = (self.x / 50.0) as i32;
                    if crate::levels::get_val(check_x + 1, check_y, points) != "#" {
                        self.possible_moves_list.push("right".to_string());
                    }
                }
            },
            Dir::Left => {
                let check_x = ((self.x - 5.0) / 50.0) as i32;
                let check_y = (self.y / 50.0) as i32;
                if crate::levels::get_val(check_x, check_y, points) != "#" {
                    self.x -= STEP_MOVE;
                }

                if self.x % 50.0 == 0.0 {
                    let check_x = (self.x / 50.0) as i32;
                    let check_y = ((self.y - 5.0) / 50.0) as i32;
                    if crate::levels::get_val(check_x, check_y, points) != "#" {
                        self.possible_moves_list.push("up".to_string());
                    }

                    let check_y = (self.y / 50.0) as i32;
                    if crate::levels::get_val(check_x, check_y + 1, points) != "#" {
                        self.possible_moves_list.push("down".to_string());
                    }
                }
            },
            Dir::Right => {
                let check_x = (self.x / 50.0) as i32;
                let check_y = (self.y / 50.0) as i32;
                if crate::levels::get_val(check_x + 1, check_y, points) != "#" {
                    self.x += STEP_MOVE;
                }

                if self.x % 50.0 == 0.0 {
                    let check_x = (self.x / 50.0) as i32;
                    let check_y = ((self.y - 5.0) / 50.0) as i32;
                    if crate::levels::get_val(check_x, check_y, points) != "#" {
                        self.possible_moves_list.push("up".to_string());
                    }

                    let check_y = (self.y / 50.0) as i32;
                    if crate::levels::get_val(check_x, check_y + 1, points) != "#" {
                        self.possible_moves_list.push("down".to_string());
                    }
                }
            },
        }

        if !self.possible_moves_list.is_empty() {
            match self.possible_moves_list.choose(&mut ::rand::rng()).unwrap().as_str() {
                "up" => self.dir = Dir::Up,
                "down" => self.dir = Dir::Down,
                "left" => self.dir = Dir::Left,
                "right" => self.dir = Dir::Right,
                _ => panic!("unknown dir"),
            };
        }
    }

    pub fn draw(&self, res: &Resources) {
        let texture = match self.dir {
            Dir::Up => &res.eyes_up,
            Dir::Down => &res.eyes_down,
            Dir::Left => &res.eyes_left,
            Dir::Right => &res.eyes_right,
        };
        draw_texture(texture, self.x, self.y, WHITE);
    }
}
