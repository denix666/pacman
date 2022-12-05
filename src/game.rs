use macroquad::prelude::*;

pub fn show_press_space_text(font: Font) {
    draw_text_ex("Press 'SPACE' to continue", 220.0, 450.0, 
        TextParams {
            font,
            font_size: 40,
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
}

impl Game {
    pub async fn new()  -> Self {
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
        }
    }
}
