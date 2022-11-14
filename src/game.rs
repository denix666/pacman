use macroquad::prelude::*;

pub struct Game {
    pub score: i32,
    pub lives: i32,
    pub level_num: i32,
    //pub time_in_seconds: f64,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            score: 0,
            lives: 0,
            level_num: 1,
            //time_in_seconds: 0.0,
        }
    }
}
