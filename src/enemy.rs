use macroquad::prelude::*;
use ::rand::seq::IndexedRandom;

use crate::game::Game;
use crate::resources::Resources;

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
    pub scared_mode: bool,
    pub speed: f32,
    pub inside_spawn: bool,
    pub dir: EnemyDir,
    pub can_cross_gate: bool,
    color_idx: usize,
    update_interval: i32,
    cur_frame: usize,
    possible_moves_list: Vec<String>,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        let color_idx = ::rand::random_range(0..=3_usize);
        let dir = match ::rand::random_range(0..=3) {
            0 => EnemyDir::Down,
            1 => EnemyDir::Left,
            2 => EnemyDir::Right,
            _ => EnemyDir::Up,
        };

        Self {
            x,
            y,
            destroyed: false,
            rect: Rect::new(0.0, 0.0, 50.0, 50.0),
            update_interval: 0,
            cur_frame: 0,
            scared_mode: false,
            dir,
            inside_spawn: true,
            possible_moves_list: vec![],
            speed: ENEMY_STEP_MOVE,
            can_cross_gate: false,
            color_idx,
        }
    }

    pub fn update(&mut self, points: &[crate::points::Point], game: &Game) {
        self.possible_moves_list.clear();

        match self.dir {
            EnemyDir::Up => {
                let check_x = (self.x / 50.0) as i32;
                let check_y = ((self.y - 5.0) / 50.0) as i32;
                if crate::levels::get_val(check_x, check_y, points) != "#" {
                    if crate::levels::get_val(check_x, check_y, points) != "-" {
                        self.y -= self.speed;
                    } else if self.can_cross_gate {
                        self.y -= self.speed;
                    }
                }

                if self.y % 50.0 == 0.0 {
                    let check_x = ((self.x - 5.0) / 50.0) as i32;
                    let check_y = (self.y / 50.0) as i32;
                    if crate::levels::get_val(check_x, check_y, points) != "#" &&
                        crate::levels::get_val(check_x, check_y, points) != "-" {
                        self.possible_moves_list.push("left".to_string());
                    }

                    let check_x = (self.x / 50.0) as i32;
                    if crate::levels::get_val(check_x + 1, check_y, points) != "#" &&
                        crate::levels::get_val(check_x + 1, check_y, points) != "-" {
                        self.possible_moves_list.push("right".to_string());
                    }
                }
            },
            EnemyDir::Down => {
                let check_x = (self.x / 50.0) as i32;
                let check_y = (self.y / 50.0) as i32;
                if crate::levels::get_val(check_x, check_y + 1, points) != "#" {
                    if crate::levels::get_val(check_x, check_y + 1, points) != "-" {
                        self.y += self.speed;
                    } else if self.can_cross_gate {
                        self.y += self.speed;
                    }
                }

                if self.y % 50.0 == 0.0 {
                    let check_x = ((self.x - 5.0) / 50.0) as i32;
                    let check_y = (self.y / 50.0) as i32;
                    if crate::levels::get_val(check_x, check_y, points) != "#" &&
                        crate::levels::get_val(check_x, check_y, points) != "-" {
                        self.possible_moves_list.push("left".to_string());
                    }

                    let check_x = (self.x / 50.0) as i32;
                    if crate::levels::get_val(check_x + 1, check_y, points) != "#" &&
                        crate::levels::get_val(check_x + 1, check_y, points) != "-" {
                        self.possible_moves_list.push("right".to_string());
                    }
                }
            },
            EnemyDir::Left => {
                let check_x = ((self.x - 5.0) / 50.0) as i32;
                let check_y = (self.y / 50.0) as i32;
                if crate::levels::get_val(check_x, check_y, points) != "#" {
                    if crate::levels::get_val(check_x, check_y, points) != "-" {
                        self.x -= self.speed;
                    } else if self.can_cross_gate {
                        self.x -= self.speed;
                    }
                }

                if self.x % 50.0 == 0.0 {
                    let check_x = (self.x / 50.0) as i32;
                    let check_y = ((self.y - 5.0) / 50.0) as i32;
                    if crate::levels::get_val(check_x, check_y, points) != "#" &&
                        crate::levels::get_val(check_x, check_y, points) != "-" {
                        self.possible_moves_list.push("up".to_string());
                    }

                    let check_y = (self.y / 50.0) as i32;
                    if crate::levels::get_val(check_x, check_y + 1, points) != "#" &&
                        crate::levels::get_val(check_x, check_y + 1, points) != "-" {
                        self.possible_moves_list.push("down".to_string());
                    }
                }
            },
            EnemyDir::Right => {
                let check_x = (self.x / 50.0) as i32;
                let check_y = (self.y / 50.0) as i32;
                if crate::levels::get_val(check_x + 1, check_y, points) != "#" {
                    if crate::levels::get_val(check_x + 1, check_y, points) != "-" {
                        self.x += self.speed;
                    } else if self.can_cross_gate {
                        self.x += self.speed;
                    }
                }

                if self.x % 50.0 == 0.0 {
                    let check_x = (self.x / 50.0) as i32;
                    let check_y = ((self.y - 5.0) / 50.0) as i32;
                    if crate::levels::get_val(check_x, check_y, points) != "#" &&
                        crate::levels::get_val(check_x, check_y, points) != "-" {
                        self.possible_moves_list.push("up".to_string());
                    }

                    let check_y = (self.y / 50.0) as i32;
                    if crate::levels::get_val(check_x, check_y + 1, points) != "#" &&
                        crate::levels::get_val(check_x, check_y + 1, points) != "-" {
                        self.possible_moves_list.push("down".to_string());
                    }
                }
            },
        }

        if crate::levels::get_val((self.x / 50.0) as i32, (self.y / 50.0) as i32, points) != "s" {
            self.inside_spawn = false;
        }

        if self.inside_spawn && self.can_cross_gate {
            if self.x >= game.spawn_gate_x && self.possible_moves_list.iter().any(|a| a == "left") {
                self.dir = EnemyDir::Left;
            } else if self.x < game.spawn_gate_x && self.possible_moves_list.iter().any(|a| a == "right") {
                self.dir = EnemyDir::Right;
            } else if self.y > game.spawn_gate_y + 50.0 && self.possible_moves_list.iter().any(|a| a == "up") {
                self.dir = EnemyDir::Up;
            } else if self.possible_moves_list.iter().any(|a| a == "down") {
                self.dir = EnemyDir::Up;
            }
        } else if !self.possible_moves_list.is_empty() {
            match self.possible_moves_list.choose(&mut ::rand::rng()).unwrap().as_str() {
                "up" => self.dir = EnemyDir::Up,
                "down" => self.dir = EnemyDir::Down,
                "left" => self.dir = EnemyDir::Left,
                "right" => self.dir = EnemyDir::Right,
                _ => panic!("unknown dir"),
            };
        }

        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn draw(&mut self, res: &Resources) {
        self.update_interval += 1;
        if self.update_interval > ANIMATION_SPEED {
            self.update_interval = 0;
            self.cur_frame += 1;
            if self.cur_frame >= 2 {
                self.cur_frame = 0;
            }
        }

        let texture = if self.scared_mode {
            match self.dir {
                EnemyDir::Up => &res.scared_up[self.cur_frame],
                EnemyDir::Down => &res.scared_down[self.cur_frame],
                EnemyDir::Left => &res.scared_left[self.cur_frame],
                EnemyDir::Right => &res.scared_right[self.cur_frame],
            }
        } else {
            let et = &res.enemy_textures[self.color_idx];
            match self.dir {
                EnemyDir::Up => &et.up[self.cur_frame],
                EnemyDir::Down => &et.down[self.cur_frame],
                EnemyDir::Left => &et.left[self.cur_frame],
                EnemyDir::Right => &et.right[self.cur_frame],
            }
        };
        draw_texture(texture, self.x, self.y, WHITE);
    }
}
