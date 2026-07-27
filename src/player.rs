use macroquad::prelude::*;

use crate::resources::{Resources, RES_WIDTH, MAP_PIXEL_WIDTH};

const ANIMATION_SPEED: i32 = 8;
pub const PLAYER_STEP_MOVE: f32 = 5.0;

pub enum PlayerDir {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub dir: PlayerDir,
    pub requested_dir: PlayerDir,
    update_interval: i32,
    cur_frame: usize,
    pub rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 550.0,
            y: 650.0,
            dir: PlayerDir::Left,
            requested_dir: PlayerDir::Left,
            update_interval: 0,
            cur_frame: 0,
            rect: Rect::new(0.0, 0.0, 50.0, 50.0),
        }
    }

    pub fn draw_lives(&self, num_of_lives: i32, res: &Resources) {
        let ly = 0.0;
        for i in 0..num_of_lives {
            let lx = (RES_WIDTH as f32 - 50.0) - i as f32 * 50.0;
            draw_texture(&res.player_right[1], lx, ly, WHITE);
        }
    }

    pub fn update(&mut self, points: &[crate::points::Point]) {
        if is_key_down(KeyCode::Left) {
            self.requested_dir = PlayerDir::Left;
        }
        if is_key_down(KeyCode::Right) {
            self.requested_dir = PlayerDir::Right;
        }
        if is_key_down(KeyCode::Up) {
            self.requested_dir = PlayerDir::Up;
        }
        if is_key_down(KeyCode::Down) {
            self.requested_dir = PlayerDir::Down;
        }

        match self.requested_dir {
            PlayerDir::Up => {
                if self.x % 50.0 == 0.0 {
                    let check_x = (self.x / 50.0) as i32;
                    let check_y = ((self.y - PLAYER_STEP_MOVE) / 50.0) as i32;
                    if crate::levels::get_val(check_x, check_y, points) != "#" {
                        self.dir = PlayerDir::Up;
                    }
                }
            },
            PlayerDir::Down => {
                if self.x % 50.0 == 0.0 {
                    let check_x = (self.x / 50.0) as i32;
                    let check_y = (self.y / 50.0) as i32;
                    if crate::levels::get_val(check_x, check_y + 1, points) != "#" &&
                        crate::levels::get_val(check_x, check_y + 1, points) != "-" {
                        self.dir = PlayerDir::Down;
                    }
                }
            },
            PlayerDir::Left => {
                if self.y % 50.0 == 0.0 {
                    let check_x = ((self.x - PLAYER_STEP_MOVE) / 50.0) as i32;
                    let check_y = (self.y / 50.0) as i32;
                    let val = crate::levels::get_val(check_x, check_y, points);
                    if val != "#" && val != "-" || val == "T" {
                        self.dir = PlayerDir::Left;
                    }
                }
            },
            PlayerDir::Right => {
                if self.y % 50.0 == 0.0 {
                    let check_x = (self.x / 50.0) as i32;
                    let check_y = (self.y / 50.0) as i32;
                    let val = crate::levels::get_val(check_x + 1, check_y, points);
                    if val != "#" && val != "-" || val == "T" {
                        self.dir = PlayerDir::Right;
                    }
                }
            },
        };

        match self.dir {
            PlayerDir::Up => {
                let check_x = (self.x / 50.0) as i32;
                let check_y = ((self.y - PLAYER_STEP_MOVE) / 50.0) as i32;
                if crate::levels::get_val(check_x, check_y, points) != "#" &&
                    crate::levels::get_val(check_x, check_y, points) != "-" {
                    self.y -= PLAYER_STEP_MOVE;
                }
            },
            PlayerDir::Down => {
                let check_x = (self.x / 50.0) as i32;
                let check_y = (self.y / 50.0) as i32;
                if crate::levels::get_val(check_x, check_y + 1, points) != "#" &&
                    crate::levels::get_val(check_x, check_y + 1, points) != "-" {
                    self.y += PLAYER_STEP_MOVE;
                }
            },
            PlayerDir::Left => {
                let check_x = ((self.x - PLAYER_STEP_MOVE) / 50.0) as i32;
                let check_y = (self.y / 50.0) as i32;
                let val = crate::levels::get_val(check_x, check_y, points);
                if val != "#" && val != "-" || val == "T" {
                    self.x -= PLAYER_STEP_MOVE;
                }
            },
            PlayerDir::Right => {
                let check_x = (self.x / 50.0) as i32;
                let check_y = (self.y / 50.0) as i32;
                let val = crate::levels::get_val(check_x + 1, check_y, points);
                if val != "#" && val != "-" || val == "T" {
                    self.x += PLAYER_STEP_MOVE;
                }
            },
        }

        // Tunnel wrap-around
        if self.x < 0.0 {
            self.x = MAP_PIXEL_WIDTH - 50.0;
        } else if self.x >= MAP_PIXEL_WIDTH {
            self.x = 0.0;
        }

        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn draw(&mut self, res: &Resources) {
        self.update_interval += 1;
        if self.update_interval > ANIMATION_SPEED {
            self.update_interval = 0;
            self.cur_frame += 1;
            if self.cur_frame >= 3 {
                self.cur_frame = 0;
            }
        }

        let texture = match self.dir {
            PlayerDir::Up => &res.player_up[self.cur_frame],
            PlayerDir::Down => &res.player_down[self.cur_frame],
            PlayerDir::Left => &res.player_left[self.cur_frame],
            PlayerDir::Right => &res.player_right[self.cur_frame],
        };
        draw_texture(texture, self.x, self.y, WHITE);
    }
}
