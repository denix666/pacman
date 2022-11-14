use macroquad::{prelude::*, audio::{PlaySoundParams, play_sound}};

mod settings;
use settings::*;

mod map;
use map::*;

mod game;
use game::*;

mod player;
use player::*;

mod coin;
use coin::*;

mod res;
use res::*;

pub enum GameState {
    Game,
    Intro,
    InitLevel,
    LevelCompleted,
}

fn window_conf() -> Conf {
    let mut title = String::from("Pacman v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    Conf {
        window_title: title
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: RES_WIDTH,
        window_height: RES_HEIGHT,
        ..Default::default()
    }
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

fn can_go_to(y: f32, x: f32, level_num: i32) -> bool {
    let lvl = match level_num {
        1  => LVL1,
        2  => LVL2,
        3  => LVL3,
        _ => {
            panic!("no such level");
        }
    };


    if lvl[(x / 50.0) as usize][(y / 50.0) as usize] != 1 {
        return true
    } else {
        return false;
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Intro;
    let mut game = Game::new().await;
    let mut map = Map::new(game.level_num).await;
    let mut player = Player::new().await;
    let mut small_coins: Vec<Coin> = Vec::new();
    let mut big_coins: Vec<Coin> = Vec::new();
    let resources = Resources::new().await;

    let pog = 1.0;
    
    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Intro => {
                if is_key_pressed(KeyCode::Space) {
                    game.score = 0;
                    game.lives = 2;
                    game.level_num = 1;
                    game_state = GameState::InitLevel;
                }
            }

            GameState::InitLevel => {
                map.lvl_num = game.level_num;
                big_coins.clear();
                small_coins.clear();
                
                // load coins
                let lvl = match game.level_num {
                    1  => LVL1,
                    2  => LVL2,
                    3  => LVL3,
                    _ => {
                        panic!("no such level");
                    }
                };
                let mut x: f32 = 0.0;
                let mut y: f32 = 0.0;
                for i in 0..lvl.len() {
                    for (_, &value) in lvl[i].iter().enumerate() {
                        if value == 0 {
                            small_coins.push(
                                Coin::new(x, y, "small").await,
                            )
                        }
                        if value == 2 {
                            big_coins.push(
                                Coin::new(x, y, "big").await,
                            )
                        }
                        x += 50.0;
                    }
                    y += 50.0;
                    x = 0.0;
                }

                game_state = GameState::Game;
            }

            GameState::LevelCompleted => {
                if is_key_pressed(KeyCode::Space) {
                    game.level_num += 1;
                    game_state = GameState::InitLevel;
                }
            }

            GameState::Game => {
                map.draw();
                draw_score(resources.font,&game.score.to_string());

                if is_key_down(KeyCode::Left) {
                    player.requested_dir = Dir::Left;
                }

                if is_key_down(KeyCode::Right) {
                    player.requested_dir = Dir::Right;
                }

                if is_key_down(KeyCode::Up) {
                    player.requested_dir = Dir::Up;
                }

                if is_key_down(KeyCode::Down) {
                    player.requested_dir = Dir::Down;
                }

                match player.requested_dir {
                    Dir::Up => {
                        if player.x % 50.0 == 0.0 {
                            if can_go_to(player.x, player.y - STEP_MOVE, game.level_num) {
                                player.dir = Dir::Up;
                            }
                        }
                    },
                    Dir::Down => {
                        if player.x % 50.0 == 0.0 {
                            if can_go_to(player.x, player.y + 50.0 + STEP_MOVE - pog, game.level_num) {
                                player.dir = Dir::Down;
                            }
                        }
                    },
                    Dir::Left => {
                        if player.y % 50.0 == 0.0 {
                            if can_go_to(player.x - STEP_MOVE, player.y, game.level_num) {
                                player.dir = Dir::Left;
                            }
                        }
                    },
                    Dir::Right => {
                        if player.y % 50.0 == 0.0 {
                            if can_go_to(player.x + 50.0 + STEP_MOVE - pog, player.y , game.level_num) {
                                player.dir = Dir::Right;
                            }
                        }
                    },
                };

                match player.dir {
                    Dir::Up => {
                        if can_go_to(player.x, player.y - STEP_MOVE, game.level_num) {
                            player.y -= STEP_MOVE;
                        }
                    },
                    Dir::Down => {
                        if can_go_to(player.x, player.y + 50.0 + STEP_MOVE - pog, game.level_num) {
                            player.y += STEP_MOVE;
                        }
                    },
                    Dir::Left => {
                        if can_go_to(player.x - STEP_MOVE, player.y, game.level_num) {
                            player.x -= STEP_MOVE;
                        }
                    },
                    Dir::Right => {
                        if can_go_to(player.x + 50.0 + STEP_MOVE - pog, player.y , game.level_num) {
                            player.x += STEP_MOVE;
                        }
                    },
                }

                for coin in &mut small_coins {
                    coin.draw();

                    if let Some(_i) = coin.rect.intersect(player.rect) {
                        coin.destroyed = true;
                        game.score += 10;
                        play_sound(resources.coin, PlaySoundParams {
                            looped: false,
                            volume: 0.2,
                        });
                    }
                }

                for coin in &mut big_coins {
                    coin.draw();

                    if let Some(_i) = coin.rect.intersect(player.rect) {
                        coin.destroyed = true;
                        game.score += 50;
                        play_sound(resources.bonus, PlaySoundParams {
                            looped: false,
                            volume: 0.2,
                        });
                    }
                }

                player.draw();

                if small_coins.len() == 0 {
                    game_state = GameState::LevelCompleted;
                }
            }
        }

        match small_coins.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                small_coins.remove(idx);
            },
            None => {},
        };

        match big_coins.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                big_coins.remove(idx);
            },
            None => {},
        };

        next_frame().await
    }
}