use macroquad::{prelude::*, audio::{Sound, load_sound_from_bytes}};

pub const RES_WIDTH: i32 = 1150;
pub const RES_HEIGHT: i32 = 750;
pub const STARTING_AMOUNT_OF_ENEMY: i32 = 3;
pub const PLAYER_START_X_POS: f32 = 550.0;
pub const PLAYER_START_Y_POS: f32 = 650.0;
pub const TARGET_FPS: f32 = 60.0;

fn tex(data: &[u8]) -> Texture2D {
    Texture2D::from_file_with_format(data, None)
}

pub struct EnemyTextures {
    pub up: [Texture2D; 2],
    pub down: [Texture2D; 2],
    pub left: [Texture2D; 2],
    pub right: [Texture2D; 2],
}

pub struct Resources {
    pub intro_texture: Texture2D,
    pub font: Font,
    // Sounds
    pub bonus_snd: Sound,
    pub coin_snd: Sound,
    pub big_coin_snd: Sound,
    pub eat_ghost_snd: Sound,
    pub siren_snd: Sound,
    pub die_snd: Sound,
    pub beginning_snd: Sound,
    pub new_live_snd: Sound,
    // Coins
    pub small_coin_texture: Texture2D,
    pub big_coin_texture: Texture2D,
    // Player
    pub player_up: [Texture2D; 3],
    pub player_down: [Texture2D; 3],
    pub player_left: [Texture2D; 3],
    pub player_right: [Texture2D; 3],
    // Enemies (4 colors: red, blue, pinc, green)
    pub enemy_textures: [EnemyTextures; 4],
    pub scared_up: [Texture2D; 2],
    pub scared_down: [Texture2D; 2],
    pub scared_left: [Texture2D; 2],
    pub scared_right: [Texture2D; 2],
    // Eyes
    pub eyes_up: Texture2D,
    pub eyes_down: Texture2D,
    pub eyes_left: Texture2D,
    pub eyes_right: Texture2D,
    // Die animation (12 frames)
    pub die_frames: [Texture2D; 12],
    // Bonus animation (15 frames)
    pub bonus_anim_frames: [Texture2D; 15],
    // Bonus items
    pub bonus_textures: [Texture2D; 4],
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            intro_texture: tex(include_bytes!("../assets/images/intro.png")),
            font: load_ttf_font_from_bytes(include_bytes!("../assets/fonts/game_font.ttf")).unwrap(),
            // Sounds
            bonus_snd: load_sound_from_bytes(include_bytes!("../assets/sounds/bonus.ogg")).await.unwrap(),
            coin_snd: load_sound_from_bytes(include_bytes!("../assets/sounds/coin.ogg")).await.unwrap(),
            big_coin_snd: load_sound_from_bytes(include_bytes!("../assets/sounds/big_coin.ogg")).await.unwrap(),
            eat_ghost_snd: load_sound_from_bytes(include_bytes!("../assets/sounds/eat_ghost.ogg")).await.unwrap(),
            siren_snd: load_sound_from_bytes(include_bytes!("../assets/sounds/siren.ogg")).await.unwrap(),
            die_snd: load_sound_from_bytes(include_bytes!("../assets/sounds/die.ogg")).await.unwrap(),
            beginning_snd: load_sound_from_bytes(include_bytes!("../assets/sounds/beginning.ogg")).await.unwrap(),
            new_live_snd: load_sound_from_bytes(include_bytes!("../assets/sounds/new_live.ogg")).await.unwrap(),
            // Coins
            small_coin_texture: tex(include_bytes!("../assets/images/small_coin.png")),
            big_coin_texture: tex(include_bytes!("../assets/images/big_coin.png")),
            // Player
            player_up: [
                tex(include_bytes!("../assets/images/player/u_0.png")),
                tex(include_bytes!("../assets/images/player/u_1.png")),
                tex(include_bytes!("../assets/images/player/u_2.png")),
            ],
            player_down: [
                tex(include_bytes!("../assets/images/player/d_0.png")),
                tex(include_bytes!("../assets/images/player/d_1.png")),
                tex(include_bytes!("../assets/images/player/d_2.png")),
            ],
            player_left: [
                tex(include_bytes!("../assets/images/player/l_0.png")),
                tex(include_bytes!("../assets/images/player/l_1.png")),
                tex(include_bytes!("../assets/images/player/l_2.png")),
            ],
            player_right: [
                tex(include_bytes!("../assets/images/player/r_0.png")),
                tex(include_bytes!("../assets/images/player/r_1.png")),
                tex(include_bytes!("../assets/images/player/r_2.png")),
            ],
            // Enemies
            enemy_textures: [
                // Red
                EnemyTextures {
                    up: [
                        tex(include_bytes!("../assets/images/enemy/red_up_0.png")),
                        tex(include_bytes!("../assets/images/enemy/red_up_1.png")),
                    ],
                    down: [
                        tex(include_bytes!("../assets/images/enemy/red_down_0.png")),
                        tex(include_bytes!("../assets/images/enemy/red_down_1.png")),
                    ],
                    left: [
                        tex(include_bytes!("../assets/images/enemy/red_left_0.png")),
                        tex(include_bytes!("../assets/images/enemy/red_left_1.png")),
                    ],
                    right: [
                        tex(include_bytes!("../assets/images/enemy/red_right_0.png")),
                        tex(include_bytes!("../assets/images/enemy/red_right_1.png")),
                    ],
                },
                // Blue
                EnemyTextures {
                    up: [
                        tex(include_bytes!("../assets/images/enemy/blue_up_0.png")),
                        tex(include_bytes!("../assets/images/enemy/blue_up_1.png")),
                    ],
                    down: [
                        tex(include_bytes!("../assets/images/enemy/blue_down_0.png")),
                        tex(include_bytes!("../assets/images/enemy/blue_down_1.png")),
                    ],
                    left: [
                        tex(include_bytes!("../assets/images/enemy/blue_left_0.png")),
                        tex(include_bytes!("../assets/images/enemy/blue_left_1.png")),
                    ],
                    right: [
                        tex(include_bytes!("../assets/images/enemy/blue_right_0.png")),
                        tex(include_bytes!("../assets/images/enemy/blue_right_1.png")),
                    ],
                },
                // Pinc
                EnemyTextures {
                    up: [
                        tex(include_bytes!("../assets/images/enemy/pinc_up_0.png")),
                        tex(include_bytes!("../assets/images/enemy/pinc_up_1.png")),
                    ],
                    down: [
                        tex(include_bytes!("../assets/images/enemy/pinc_down_0.png")),
                        tex(include_bytes!("../assets/images/enemy/pinc_down_1.png")),
                    ],
                    left: [
                        tex(include_bytes!("../assets/images/enemy/pinc_left_0.png")),
                        tex(include_bytes!("../assets/images/enemy/pinc_left_1.png")),
                    ],
                    right: [
                        tex(include_bytes!("../assets/images/enemy/pinc_right_0.png")),
                        tex(include_bytes!("../assets/images/enemy/pinc_right_1.png")),
                    ],
                },
                // Green
                EnemyTextures {
                    up: [
                        tex(include_bytes!("../assets/images/enemy/green_up_0.png")),
                        tex(include_bytes!("../assets/images/enemy/green_up_1.png")),
                    ],
                    down: [
                        tex(include_bytes!("../assets/images/enemy/green_down_0.png")),
                        tex(include_bytes!("../assets/images/enemy/green_down_1.png")),
                    ],
                    left: [
                        tex(include_bytes!("../assets/images/enemy/green_left_0.png")),
                        tex(include_bytes!("../assets/images/enemy/green_left_1.png")),
                    ],
                    right: [
                        tex(include_bytes!("../assets/images/enemy/green_right_0.png")),
                        tex(include_bytes!("../assets/images/enemy/green_right_1.png")),
                    ],
                },
            ],
            // Scared
            scared_up: [
                tex(include_bytes!("../assets/images/enemy/scared_up_0.png")),
                tex(include_bytes!("../assets/images/enemy/scared_up_1.png")),
            ],
            scared_down: [
                tex(include_bytes!("../assets/images/enemy/scared_down_0.png")),
                tex(include_bytes!("../assets/images/enemy/scared_down_1.png")),
            ],
            scared_left: [
                tex(include_bytes!("../assets/images/enemy/scared_left_0.png")),
                tex(include_bytes!("../assets/images/enemy/scared_left_1.png")),
            ],
            scared_right: [
                tex(include_bytes!("../assets/images/enemy/scared_right_0.png")),
                tex(include_bytes!("../assets/images/enemy/scared_right_1.png")),
            ],
            // Eyes
            eyes_up: tex(include_bytes!("../assets/images/eyes/up.png")),
            eyes_down: tex(include_bytes!("../assets/images/eyes/down.png")),
            eyes_left: tex(include_bytes!("../assets/images/eyes/left.png")),
            eyes_right: tex(include_bytes!("../assets/images/eyes/right.png")),
            // Die animation
            die_frames: [
                tex(include_bytes!("../assets/images/player_die/0.png")),
                tex(include_bytes!("../assets/images/player_die/1.png")),
                tex(include_bytes!("../assets/images/player_die/2.png")),
                tex(include_bytes!("../assets/images/player_die/3.png")),
                tex(include_bytes!("../assets/images/player_die/4.png")),
                tex(include_bytes!("../assets/images/player_die/5.png")),
                tex(include_bytes!("../assets/images/player_die/6.png")),
                tex(include_bytes!("../assets/images/player_die/7.png")),
                tex(include_bytes!("../assets/images/player_die/8.png")),
                tex(include_bytes!("../assets/images/player_die/9.png")),
                tex(include_bytes!("../assets/images/player_die/10.png")),
                tex(include_bytes!("../assets/images/player_die/11.png")),
            ],
            // Bonus animation
            bonus_anim_frames: [
                tex(include_bytes!("../assets/images/bonus_animation/0.png")),
                tex(include_bytes!("../assets/images/bonus_animation/1.png")),
                tex(include_bytes!("../assets/images/bonus_animation/2.png")),
                tex(include_bytes!("../assets/images/bonus_animation/3.png")),
                tex(include_bytes!("../assets/images/bonus_animation/4.png")),
                tex(include_bytes!("../assets/images/bonus_animation/5.png")),
                tex(include_bytes!("../assets/images/bonus_animation/6.png")),
                tex(include_bytes!("../assets/images/bonus_animation/7.png")),
                tex(include_bytes!("../assets/images/bonus_animation/8.png")),
                tex(include_bytes!("../assets/images/bonus_animation/9.png")),
                tex(include_bytes!("../assets/images/bonus_animation/10.png")),
                tex(include_bytes!("../assets/images/bonus_animation/11.png")),
                tex(include_bytes!("../assets/images/bonus_animation/12.png")),
                tex(include_bytes!("../assets/images/bonus_animation/13.png")),
                tex(include_bytes!("../assets/images/bonus_animation/14.png")),
            ],
            // Bonus items (apple, cake, cherry, strawberry)
            bonus_textures: [
                tex(include_bytes!("../assets/images/bonus/apple.png")),
                tex(include_bytes!("../assets/images/bonus/cake.png")),
                tex(include_bytes!("../assets/images/bonus/cherry.png")),
                tex(include_bytes!("../assets/images/bonus/strawberry.png")),
            ],
        }
    }
}
