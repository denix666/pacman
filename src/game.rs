use macroquad::prelude::*;

pub struct Game {
    pub score: i32,
    pub lives: i32,
    pub level_num: i32,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            score: 0,
            lives: 0,
            level_num: 1,
        }
    }
}
