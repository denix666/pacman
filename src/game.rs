use macroquad::prelude::*;

pub struct Game {
    pub score: i32,
    pub lives: i32,
    pub level_num: i32,
    pub scared_mode: bool,
    pub scared_mode_started_at: f64,
    pub siren_played: bool,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            score: 0,
            lives: 0,
            level_num: 1,
            scared_mode: false,
            scared_mode_started_at: 0.0,
            siren_played: false,
        }
    }
}
