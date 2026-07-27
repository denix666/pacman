use macroquad::prelude::*;

use crate::resources::Resources;

const ANIMATION_SPEED: i32 = 4;

pub struct DieAnimation {
    pub x: f32,
    pub y: f32,
    update_interval: i32,
    cur_frame: usize,
    pub destroyed: bool,
}

impl DieAnimation {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            update_interval: 0,
            cur_frame: 0,
            destroyed: false,
        }
    }

    pub fn draw(&mut self, res: &Resources) {
        if !self.destroyed {
            draw_texture(&res.die_frames[self.cur_frame], self.x, self.y, WHITE);
            self.update_interval += 1;
            if self.update_interval > ANIMATION_SPEED {
                self.update_interval = 0;
                self.cur_frame += 1;
                if self.cur_frame >= 12 {
                    self.destroyed = true;
                }
            }
        }
    }
}
