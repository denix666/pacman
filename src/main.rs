use macroquad::{prelude::*, audio::{PlaySoundParams, play_sound}};

extern crate rand;
use rand::{Rng};

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

mod messages;
use messages::*;

mod enemy;
use enemy::*;

pub enum GameState {
    Game,
    Intro,
    InitLevel,
    LevelCompleted,
    LevelFailed,
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
    let mut enemies: Vec<Enemy> = Vec::new();
    let resources = Resources::new().await;

    let pog = 1.0;
    
    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Intro => {
                draw_texture(resources.intro_texture, 0.0, 0.0, WHITE);
                show_intro_text(resources.font);

                if is_key_pressed(KeyCode::Space) {
                    game.score = 0;
                    game.lives = 2;
                    game.level_num = 1;
                    game_state = GameState::InitLevel;
                }
            }

            GameState::LevelFailed => {
                map.draw();
                player.status = Status::Died;
                player.draw();
                player.draw_lives(game.lives);
                draw_score(resources.font,&game.score.to_string());
                

                if is_key_pressed(KeyCode::Space) {
                    if game.lives > 0 {
                        game.lives -= 1;
                        player.x = 550.0;
                        player.y = 650.0;
                        player.status = Status::Playing;
                        game_state = GameState::Game;
                    } else {
                        // gameover
                    }
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
                            );
                        }
                        if value == 2 {
                            big_coins.push(
                                Coin::new(x, y, "big").await,
                            );
                        }
                        x += 50.0;
                    }
                    y += 50.0;
                    x = 0.0;
                }

                // load enemies
                let mut item_placed_successfully: bool;
                for _ in 1..=STARTING_AMOUNT_OF_ENEMY + game.level_num {
                    item_placed_successfully = false;
                    while !item_placed_successfully {
                        let y = rand::thread_rng().gen_range(0..=10);
                        let x = rand::thread_rng().gen_range(0..=22);
                        
                        if lvl[y as usize][x as usize] == 0 {
                            let mut enemy_in_this_place_already_exist = false;
                            for en in &mut enemies {
                                if en.x == x as f32 * 50.0 && en.y == y as f32 * 50.0 {
                                    enemy_in_this_place_already_exist = true;
                                    break;
                                }
                            }
                            if !enemy_in_this_place_already_exist {
                                enemies.push(
                                    Enemy::new(x as f32 * 50.0, y as f32 * 50.0).await,
                                );
                                item_placed_successfully = true;
                            }
                        }
                    }
                }

                game_state = GameState::Game;
            }

            GameState::LevelCompleted => {
                map.draw();
                show_level_completed_text(resources.font);
                if is_key_pressed(KeyCode::Space) {
                    game.level_num += 1;
                    game_state = GameState::InitLevel;
                }
            }

            GameState::Game => {
                map.draw();
                draw_score(resources.font,&game.score.to_string());

                if is_key_down(KeyCode::Left) {
                    player.requested_dir = PlayerDir::Left;
                }

                if is_key_down(KeyCode::Right) {
                    player.requested_dir = PlayerDir::Right;
                }

                if is_key_down(KeyCode::Up) {
                    player.requested_dir = PlayerDir::Up;
                }

                if is_key_down(KeyCode::Down) {
                    player.requested_dir = PlayerDir::Down;
                }

                match player.requested_dir {
                    PlayerDir::Up => {
                        if player.x % 50.0 == 0.0 {
                            if can_go_to(player.x, player.y - PLAYER_STEP_MOVE, game.level_num) {
                                player.dir = PlayerDir::Up;
                            }
                        }
                    },
                    PlayerDir::Down => {
                        if player.x % 50.0 == 0.0 {
                            if can_go_to(player.x, player.y + 50.0 + PLAYER_STEP_MOVE - pog, game.level_num) {
                                player.dir = PlayerDir::Down;
                            }
                        }
                    },
                    PlayerDir::Left => {
                        if player.y % 50.0 == 0.0 {
                            if can_go_to(player.x - PLAYER_STEP_MOVE, player.y, game.level_num) {
                                player.dir = PlayerDir::Left;
                            }
                        }
                    },
                    PlayerDir::Right => {
                        if player.y % 50.0 == 0.0 {
                            if can_go_to(player.x + 50.0 + PLAYER_STEP_MOVE - pog, player.y , game.level_num) {
                                player.dir = PlayerDir::Right;
                            }
                        }
                    },
                };

                match player.dir {
                    PlayerDir::Up => {
                        if can_go_to(player.x, player.y - PLAYER_STEP_MOVE, game.level_num) {
                            player.y -= PLAYER_STEP_MOVE;
                        }
                    },
                    PlayerDir::Down => {
                        if can_go_to(player.x, player.y + 50.0 + PLAYER_STEP_MOVE - pog, game.level_num) {
                            player.y += PLAYER_STEP_MOVE;
                        }
                    },
                    PlayerDir::Left => {
                        if can_go_to(player.x - PLAYER_STEP_MOVE, player.y, game.level_num) {
                            player.x -= PLAYER_STEP_MOVE;
                        }
                    },
                    PlayerDir::Right => {
                        if can_go_to(player.x + 50.0 + PLAYER_STEP_MOVE - pog, player.y , game.level_num) {
                            player.x += PLAYER_STEP_MOVE;
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
                player.draw_lives(game.lives);

                for enemy in &mut enemies {
                    enemy.draw();
                    if let Some(_i) = enemy.rect.intersect(player.rect) {
                        game_state = GameState::LevelFailed;
                    }
                }

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

        match enemies.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                enemies.remove(idx);
            },
            None => {},
        };

        next_frame().await
    }
}