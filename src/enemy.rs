use macroquad::prelude::*;
use ::rand::seq::IndexedRandom;

use crate::game::Game;
use crate::points::Point;
use crate::resources::{Resources, MAP_WIDTH};

const ANIMATION_SPEED: i32 = 8;
const TILE_SIZE: f32 = 50.0;

#[derive(Clone, Copy, PartialEq)]
pub enum EnemyDir {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl EnemyDir {
    fn opposite(self) -> EnemyDir {
        match self {
            EnemyDir::Up => EnemyDir::Down,
            EnemyDir::Down => EnemyDir::Up,
            EnemyDir::Left => EnemyDir::Right,
            EnemyDir::Right => EnemyDir::Left,
            EnemyDir::None => EnemyDir::None,
        }
    }

    fn dx(self) -> i32 {
        match self {
            EnemyDir::Left => -1,
            EnemyDir::Right => 1,
            _ => 0,
        }
    }

    fn dy(self) -> i32 {
        match self {
            EnemyDir::Up => -1,
            EnemyDir::Down => 1,
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum GhostState {
    InHouse,
    Exiting,
    Active,
    Eyes,
    Entering,
}

pub struct Enemy {
    pub tile_x: i32,
    pub tile_y: i32,
    pub offset: f32,
    pub dir: EnemyDir,
    pub next_dir: EnemyDir,
    pub rect: Rect,
    pub scared_mode: bool,
    pub immune_to_scared: bool,
    pub state: GhostState,
    pub color_idx: usize,
    update_interval: i32,
    cur_frame: usize,
    home_tile_x: i32,
    home_tile_y: i32,
    pixel_x: f32,
    pixel_y: f32,
    respawn_timer: f64,
}

impl Enemy {
    pub fn new(x: f32, y: f32, color_idx: usize) -> Self {
        let tile_x = (x / TILE_SIZE) as i32;
        let tile_y = (y / TILE_SIZE) as i32;
        Self {
            tile_x,
            tile_y,
            offset: 0.0,
            dir: EnemyDir::None,
            next_dir: EnemyDir::None,
            rect: Rect::new(x, y, TILE_SIZE, TILE_SIZE),
            update_interval: 0,
            cur_frame: 0,
            scared_mode: false,
            immune_to_scared: false,
            state: GhostState::InHouse,
            color_idx,
            home_tile_x: tile_x,
            home_tile_y: tile_y,
            pixel_x: x,
            pixel_y: y,
            respawn_timer: 0.0,
        }
    }

    fn speed(&self) -> f32 {
        match self.state {
            GhostState::Eyes | GhostState::Entering => 10.0,
            GhostState::Active if self.scared_mode => 2.5,
            _ => 5.0,
        }
    }

    fn can_pass(val: &str, state: GhostState) -> bool {
        match val {
            "#" => false,
            "-" => matches!(state, GhostState::Exiting | GhostState::Eyes | GhostState::Entering | GhostState::InHouse),
            "T" => true,
            "empty" => false,
            _ => true,
        }
    }

    fn get_available_dirs_at(&self, tx: i32, ty: i32, points: &[Point]) -> Vec<EnemyDir> {
        let mut dirs = Vec::new();

        let up_val = crate::levels::get_val(tx, ty - 1, points);
        if Self::can_pass(&up_val, self.state) {
            dirs.push(EnemyDir::Up);
        }

        let down_val = crate::levels::get_val(tx, ty + 1, points);
        if Self::can_pass(&down_val, self.state) {
            dirs.push(EnemyDir::Down);
        }

        let left_val = crate::levels::get_val(tx - 1, ty, points);
        if Self::can_pass(&left_val, self.state) || left_val == "T" {
            dirs.push(EnemyDir::Left);
        }

        let right_val = crate::levels::get_val(tx + 1, ty, points);
        if Self::can_pass(&right_val, self.state) || right_val == "T" {
            dirs.push(EnemyDir::Right);
        }

        dirs
    }

    fn target_tile(&self, game: &Game, player_x: f32, player_y: f32, player_dir: &crate::player::PlayerDir) -> (i32, i32) {
        let px = (player_x / TILE_SIZE) as i32;
        let py = (player_y / TILE_SIZE) as i32;

        match self.state {
            GhostState::Eyes | GhostState::Entering => {
                let gx = (game.spawn_gate_x / TILE_SIZE) as i32;
                let gy = (game.spawn_gate_y / TILE_SIZE) as i32;
                (gx, gy)
            },
            GhostState::Active if self.scared_mode => {
                (px, py)
            },
            GhostState::Active => {
                match self.color_idx {
                    0 => (px, py),
                    1 => {
                        let (ox, oy) = match player_dir {
                            crate::player::PlayerDir::Up => (0, -4),
                            crate::player::PlayerDir::Down => (0, 4),
                            crate::player::PlayerDir::Left => (-4, 0),
                            crate::player::PlayerDir::Right => (4, 0),
                        };
                        (px + ox, py + oy)
                    },
                    2 => {
                        let (ox, oy) = match player_dir {
                            crate::player::PlayerDir::Up => (0, -2),
                            crate::player::PlayerDir::Down => (0, 2),
                            crate::player::PlayerDir::Left => (-2, 0),
                            crate::player::PlayerDir::Right => (2, 0),
                        };
                        let ahead_x = px + ox;
                        let ahead_y = py + oy;
                        (ahead_x * 2 - self.tile_x, ahead_y * 2 - self.tile_y)
                    },
                    _ => {
                        let dist_sq = (px - self.tile_x).pow(2) + (py - self.tile_y).pow(2);
                        if dist_sq > 64 {
                            (px, py)
                        } else {
                            (0, 14)
                        }
                    },
                }
            },
            _ => (self.tile_x, self.tile_y),
        }
    }

    fn choose_dir_toward(&self, target_x: i32, target_y: i32, available: &[EnemyDir]) -> EnemyDir {
        if available.is_empty() {
            return self.dir;
        }

        let opposite = self.dir.opposite();
        let filtered: Vec<EnemyDir> = available.iter()
            .filter(|&&d| d != opposite)
            .copied()
            .collect();

        let choices = if filtered.is_empty() { available } else { &filtered };

        let mut best_dir = choices[0];
        let mut best_dist = i32::MAX;

        for &d in choices {
            let nx = self.tile_x + d.dx();
            let ny = self.tile_y + d.dy();
            let dist = (nx - target_x).pow(2) + (ny - target_y).pow(2);
            if dist < best_dist {
                best_dist = dist;
                best_dir = d;
            }
        }

        best_dir
    }

    fn decide_direction(&mut self, points: &[Point], game: &Game, player_x: f32, player_y: f32, player_dir: &crate::player::PlayerDir) {
        let available = self.get_available_dirs_at(self.tile_x, self.tile_y, points);

        if available.is_empty() {
            self.next_dir = EnemyDir::None;
            return;
        }

        let opposite = self.dir.opposite();
        let forward_options: Vec<EnemyDir> = available.iter()
            .filter(|&&d| d != opposite)
            .copied()
            .collect();

        let at_dead_end = forward_options.is_empty();
        let can_continue = forward_options.contains(&self.dir);
        let at_intersection = forward_options.len() > 1;

        if at_dead_end {
            self.next_dir = opposite;
        } else if at_intersection || !can_continue {
            if self.scared_mode {
                self.next_dir = *forward_options.choose(&mut ::rand::rng()).unwrap();
            } else {
                let (tx, ty) = self.target_tile(game, player_x, player_y, player_dir);
                self.next_dir = self.choose_dir_toward(tx, ty, &available);
            }
        } else {
            self.next_dir = self.dir;
        }
    }

    pub fn update(&mut self, points: &[Point], game: &Game, player_x: f32, player_y: f32, player_dir: &crate::player::PlayerDir) {
        match self.state {
            GhostState::InHouse => {
                if self.respawn_timer > 0.0 && get_time() - self.respawn_timer > 3.0 {
                    self.respawn_timer = 0.0;
                    self.state = GhostState::Exiting;
                    return;
                }

                if self.dir == EnemyDir::Up || self.dir == EnemyDir::None {
                    self.dir = EnemyDir::Up;
                    let home_px = self.home_tile_y as f32 * TILE_SIZE;
                    if self.pixel_y <= home_px - 10.0 {
                        self.dir = EnemyDir::Down;
                    } else {
                        self.pixel_y -= 2.0;
                    }
                } else {
                    let home_px = self.home_tile_y as f32 * TILE_SIZE;
                    if self.pixel_y >= home_px + 10.0 {
                        self.dir = EnemyDir::Up;
                    } else {
                        self.pixel_y += 2.0;
                    }
                }
                self.pixel_x = self.tile_x as f32 * TILE_SIZE;
            },
            GhostState::Exiting => {
                let gate_x = game.spawn_gate_x;
                let gate_y = game.spawn_gate_y;
                let target_y = gate_y - TILE_SIZE;

                if (self.pixel_x - gate_x).abs() > 2.0 {
                    if self.pixel_x < gate_x {
                        self.pixel_x += 5.0_f32.min(gate_x - self.pixel_x);
                        self.dir = EnemyDir::Right;
                    } else {
                        self.pixel_x -= 5.0_f32.min(self.pixel_x - gate_x);
                        self.dir = EnemyDir::Left;
                    }
                } else {
                    self.pixel_x = gate_x;
                    if self.pixel_y > target_y {
                        self.pixel_y -= 5.0_f32.min(self.pixel_y - target_y);
                        self.dir = EnemyDir::Up;
                    } else {
                        self.pixel_y = target_y;
                        self.tile_x = (self.pixel_x / TILE_SIZE).round() as i32;
                        self.tile_y = (self.pixel_y / TILE_SIZE).round() as i32;
                        self.pixel_x = self.tile_x as f32 * TILE_SIZE;
                        self.pixel_y = self.tile_y as f32 * TILE_SIZE;
                        self.offset = 0.0;
                        self.state = GhostState::Active;
                        self.scared_mode = false;
                        self.immune_to_scared = true;
                        self.dir = EnemyDir::Left;
                        self.next_dir = EnemyDir::Left;
                    }
                }
            },
            GhostState::Active | GhostState::Eyes => {
                let spd = self.speed();

                if self.offset == 0.0 {
                    self.decide_direction(points, game, player_x, player_y, player_dir);
                    self.dir = self.next_dir;

                    if self.state == GhostState::Eyes {
                        let gx = (game.spawn_gate_x / TILE_SIZE) as i32;
                        let gy = (game.spawn_gate_y / TILE_SIZE) as i32;
                        if self.tile_x == gx && self.tile_y == gy {
                            self.state = GhostState::Entering;
                            self.dir = EnemyDir::Down;
                            self.pixel_y = self.tile_y as f32 * TILE_SIZE;
                            self.pixel_x = self.tile_x as f32 * TILE_SIZE;
                            return;
                        }
                    }

                    if self.dir == EnemyDir::None {
                        return;
                    }
                }

                self.offset += spd;

                if self.offset >= TILE_SIZE {
                    self.offset = 0.0;
                    let mut new_tx = self.tile_x + self.dir.dx();
                    let new_ty = self.tile_y + self.dir.dy();

                    // Tunnel wrap
                    if new_tx < 0 {
                        new_tx = MAP_WIDTH - 1;
                    } else if new_tx >= MAP_WIDTH {
                        new_tx = 0;
                    }

                    self.tile_x = new_tx;
                    self.tile_y = new_ty;
                }

                let progress = self.offset / TILE_SIZE;
                self.pixel_x = (self.tile_x as f32 + progress * self.dir.dx() as f32) * TILE_SIZE;
                self.pixel_y = (self.tile_y as f32 + progress * self.dir.dy() as f32) * TILE_SIZE;
            },
            GhostState::Entering => {
                let home_py = self.home_tile_y as f32 * TILE_SIZE;
                let home_px = self.home_tile_x as f32 * TILE_SIZE;

                if self.pixel_y < home_py {
                    self.pixel_y += 5.0_f32.min(home_py - self.pixel_y);
                    self.dir = EnemyDir::Down;
                } else if (self.pixel_x - home_px).abs() > 2.0 {
                    if self.pixel_x < home_px {
                        self.pixel_x += 5.0_f32.min(home_px - self.pixel_x);
                        self.dir = EnemyDir::Right;
                    } else {
                        self.pixel_x -= 5.0_f32.min(self.pixel_x - home_px);
                        self.dir = EnemyDir::Left;
                    }
                } else {
                    self.pixel_x = home_px;
                    self.pixel_y = home_py;
                    self.tile_x = self.home_tile_x;
                    self.tile_y = self.home_tile_y;
                    self.offset = 0.0;
                    self.state = GhostState::InHouse;
                    self.scared_mode = false;
                    self.respawn_timer = get_time();
                    self.dir = EnemyDir::Up;
                }
            },
        }

        self.rect.x = self.pixel_x;
        self.rect.y = self.pixel_y;
    }

    pub fn release(&mut self) {
        if self.state == GhostState::InHouse {
            self.state = GhostState::Exiting;
        }
    }

    pub fn become_eyes(&mut self) {
        self.state = GhostState::Eyes;
        self.scared_mode = false;
        self.immune_to_scared = true;
        self.tile_x = (self.pixel_x / TILE_SIZE).round() as i32;
        self.tile_y = (self.pixel_y / TILE_SIZE).round() as i32;
        self.pixel_x = self.tile_x as f32 * TILE_SIZE;
        self.pixel_y = self.tile_y as f32 * TILE_SIZE;
        self.offset = 0.0;
    }

    pub fn is_collidable(&self) -> bool {
        self.state == GhostState::Active
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

        let draw_x = self.pixel_x;
        let draw_y = self.pixel_y;

        match self.state {
            GhostState::Eyes | GhostState::Entering => {
                let texture = match self.dir {
                    EnemyDir::Up => &res.eyes_up,
                    EnemyDir::Down => &res.eyes_down,
                    EnemyDir::Left => &res.eyes_left,
                    EnemyDir::Right | EnemyDir::None => &res.eyes_right,
                };
                draw_texture(texture, draw_x, draw_y, WHITE);
            },
            _ => {
                let dir_for_draw = if self.dir == EnemyDir::None { EnemyDir::Left } else { self.dir };
                let texture = if self.scared_mode {
                    match dir_for_draw {
                        EnemyDir::Up => &res.scared_up[self.cur_frame],
                        EnemyDir::Down => &res.scared_down[self.cur_frame],
                        EnemyDir::Left => &res.scared_left[self.cur_frame],
                        _ => &res.scared_right[self.cur_frame],
                    }
                } else {
                    let et = &res.enemy_textures[self.color_idx];
                    match dir_for_draw {
                        EnemyDir::Up => &et.up[self.cur_frame],
                        EnemyDir::Down => &et.down[self.cur_frame],
                        EnemyDir::Left => &et.left[self.cur_frame],
                        _ => &et.right[self.cur_frame],
                    }
                };
                draw_texture(texture, draw_x, draw_y, WHITE);
            },
        }
    }
}
