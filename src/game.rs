use macroquad::prelude::*;

use crate::resources::STARTING_AMOUNT_OF_ENEMY;

pub fn show_press_space_text(font: &Font) {
    draw_text_ex("Press 'SPACE' to continue", 320.0, 550.0,
        TextParams {
            font: Some(font),
            font_size: 30,
            color: WHITE,
            ..Default::default()
        },
    );
}

pub struct Game {
    pub lvl_num: i32,
    pub score: i32,
    pub lives: i32,
    pub scared_mode: bool,
    pub scared_mode_started_at: f64,
    pub siren_played: bool,
    pub last_bonus_was_at: f64,
    pub spawn_gate_x: f32,
    pub spawn_gate_y: f32,
    pub amount_of_enemy: i32,
    pub next_life_at: i32,
    pub high_score: i32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            lvl_num: 0,
            score: 0,
            lives: 0,
            scared_mode: false,
            scared_mode_started_at: 0.0,
            siren_played: false,
            last_bonus_was_at: 0.0,
            spawn_gate_x: 0.0,
            spawn_gate_y: 0.0,
            amount_of_enemy: STARTING_AMOUNT_OF_ENEMY,
            next_life_at: 10000,
            high_score: 0,
        }
    }

    pub fn update_high_score(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }
}
