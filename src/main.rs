use macroquad::{prelude::*, audio::{PlaySoundParams, play_sound}};

extern crate rand;
use rand::{Rng};
use rand::seq::SliceRandom;

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

mod die_animation;
use die_animation::*;

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
        4  => LVL1,
        5  => LVL2,
        6  => LVL3,
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
    let mut animations: Vec<DieAnimation> = Vec::new();

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
                for coin in &mut small_coins {
                    coin.draw();
                }
                for coin in &mut big_coins {
                    coin.draw();
                }
                for en in &mut enemies {
                    en.draw();
                }
                
                player.draw_lives(game.lives);
                draw_score(resources.font,&game.score.to_string());

                for animation in &mut animations {
                    animation.draw();
                }
                
                if animations.len() == 0 && is_key_pressed(KeyCode::Space) {
                    if game.lives > 0 {
                        game.lives -= 1;
                        player.x = 550.0;
                        player.y = 650.0;
                        player.dir = PlayerDir::Left;
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
                enemies.clear();
                game.scared_mode = false;
                
                // load coins
                let lvl = match game.level_num {
                    1  => LVL1,
                    2  => LVL2,
                    3  => LVL3,
                    4  => LVL1,
                    5  => LVL2,
                    6  => LVL3,
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
                    player.x = 550.0;
                    player.y = 650.0;
                    player.dir = PlayerDir::Left;
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
                        for enemy in &mut enemies {
                            enemy.enemy_mode = EnemyMode::Scared;
                        }
                        game.scared_mode = true;
                        game.scared_mode_started_at = get_time();
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
                    enemy.possible_moves_list.clear();
                    
                    match enemy.dir {
                        EnemyDir::Up => {
                            if can_go_to(enemy.x, enemy.y - enemy.speed, game.level_num) {
                                enemy.y -= enemy.speed;
                            }

                            if enemy.y % 50.0 == 0.0 {
                                if can_go_to(enemy.x - enemy.speed, enemy.y, game.level_num) { //Left
                                    enemy.possible_moves_list.push("left".to_string());
                                }
                                if can_go_to(enemy.x + 50.0 + enemy.speed, enemy.y, game.level_num) { //Right
                                    enemy.possible_moves_list.push("right".to_string());
                                }
                            }
                        },
                        EnemyDir::Down => {
                            if can_go_to(enemy.x, enemy.y + 50.0 + enemy.speed - pog, game.level_num) {
                                enemy.y += enemy.speed;
                            }
                            
                            if enemy.y % 50.0 == 0.0 {
                                if can_go_to(enemy.x - enemy.speed, enemy.y, game.level_num) { //Left
                                    enemy.possible_moves_list.push("left".to_string());
                                }
                                if can_go_to(enemy.x + 50.0 + enemy.speed, enemy.y, game.level_num) { //Right
                                    enemy.possible_moves_list.push("right".to_string());
                                }
                            }
                        },
                        EnemyDir::Left => {
                            if can_go_to(enemy.x - enemy.speed, enemy.y, game.level_num) {
                                enemy.x -= enemy.speed;
                            }

                            if enemy.x % 50.0 == 0.0 {
                                if can_go_to(enemy.x, enemy.y - enemy.speed, game.level_num) { //Up
                                    enemy.possible_moves_list.push("up".to_string());
                                }
                                if can_go_to(enemy.x, enemy.y + 50.0 + enemy.speed, game.level_num) { //Down
                                    enemy.possible_moves_list.push("down".to_string());
                                }
                            }
                        },
                        EnemyDir::Right => {
                            if can_go_to(enemy.x + 50.0 + enemy.speed - pog, enemy.y , game.level_num) {
                                enemy.x += enemy.speed;
                            }

                            if enemy.x % 50.0 == 0.0 {
                                if can_go_to(enemy.x, enemy.y - enemy.speed, game.level_num) { //Up
                                    enemy.possible_moves_list.push("up".to_string());
                                }
                                if can_go_to(enemy.x, enemy.y + 50.0 + enemy.speed, game.level_num) { //Down
                                    enemy.possible_moves_list.push("down".to_string());
                                }
                            }
                        },
                    }

                    if enemy.possible_moves_list.len() > 0 {
                        // let d = "up".to_string();
                        // if enemy.possible_moves_list.contains(&d) {
                        //     if enemy.y >= player.y {
                        //         for _ in 0..7 {
                        //             enemy.possible_moves_list.push("up".to_string());
                        //         }
                        //     }
                        // }

                        // let d = "down".to_string();
                        // if enemy.possible_moves_list.contains(&d) {
                        //     if enemy.y <= player.y {
                        //         for _ in 0..7 {
                        //             enemy.possible_moves_list.push("down".to_string());
                        //         }
                        //     }
                        // }

                        // let d = "right".to_string();
                        // if enemy.possible_moves_list.contains(&d) {
                        //     if enemy.x <= player.x {
                        //         for _ in 0..7 {
                        //             enemy.possible_moves_list.push("right".to_string());
                        //         }
                        //     }
                        // }

                        // let d = "left".to_string();
                        // if enemy.possible_moves_list.contains(&d) {
                        //     if enemy.x >= player.x {
                        //         for _ in 0..7 {
                        //             enemy.possible_moves_list.push("left".to_string());
                        //         }
                        //     }
                        // }


                        match enemy.possible_moves_list.choose(&mut rand::thread_rng()).unwrap().as_str() {
                            "up" => {
                                enemy.dir = EnemyDir::Up;
                            },
                            "down" => {
                                enemy.dir = EnemyDir::Down;
                            },
                            "left" => {
                                enemy.dir = EnemyDir::Left;
                            },
                            "right" => {
                                enemy.dir = EnemyDir::Right;
                            },
                            _ => {
                                panic!("unknown dir");
                            }
                        };
                    } else {
                        println!("fff");
                    }
                    
                    
                    if let Some(_i) = enemy.rect.intersect(player.rect) {
                        match enemy.enemy_mode {
                            EnemyMode::Normal => {
                                animations.push(
                                    DieAnimation::new(player.x, player.y).await,
                                );
                                game_state = GameState::LevelFailed;
                            },
                            EnemyMode::Scared => {
                                play_sound(resources.eat_ghost, PlaySoundParams {
                                    looped: false,
                                    volume: 0.2,
                                });
                                game.score += 150;
                                enemy.enemy_mode = EnemyMode::Eyes;
                                enemy.speed = 10.0;
                            },
                            EnemyMode::Eyes => {},
                        };
                    }

                    enemy.draw();
                }

                if get_time() - game.scared_mode_started_at > 4.0 {
                    if !game.siren_played && game.scared_mode {
                        play_sound(resources.siren, PlaySoundParams {
                            looped: false,
                            volume: 0.4,
                        });
                        game.siren_played = true;
                    }
                }
                if get_time() - game.scared_mode_started_at > 6.0 {
                    game.scared_mode = false;
                    for enemy in &mut enemies {
                        enemy.enemy_mode = EnemyMode::Normal;
                        enemy.speed = 5.0;
                    }
                    game.siren_played = false;
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

        match animations.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                animations.remove(idx);
            },
            None => {},
        };

        next_frame().await
    }
}