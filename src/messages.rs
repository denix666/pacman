use macroquad::prelude::*;

pub fn show_intro_text(font: Font) {
    draw_text_ex("Press 'SPACE' to start", 280.0, 450.0, 
        TextParams {
            font,
            font_size: 40,
            color: WHITE,
            ..Default::default()
        },
    );
}

pub fn show_level_completed_text(font: Font) {
    draw_text_ex("LEVEL COMPLETED!!!", 300.0, 350.0, 
        TextParams {
            font,
            font_size: 40,
            color: WHITE,
            ..Default::default()
        },
    );
    
    draw_text_ex("Press 'SPACE' to continue", 220.0, 450.0, 
        TextParams {
            font,
            font_size: 40,
            color: WHITE,
            ..Default::default()
        },
    );
}

pub fn draw_score(font: Font, score: &str) {
    draw_text_ex("SCORE: ", 7.0, 40.0, 
        TextParams {
            font,
            font_size: 50,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(score, 250.0, 40.0, 
        TextParams {
            font,
            font_size: 50,
            color: MAGENTA,
            ..Default::default()
        },
    );
}