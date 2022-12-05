use macroquad::{prelude::*, audio::{PlaySoundParams, play_sound}};
extern crate rand;
use rand::{Rng};

mod resources;
use resources::*;

mod points;
use points::*;

mod die_animation;
use die_animation::*;

mod levels;
use levels::*;

mod eyes;
use eyes::*;

mod game;
use game::*;

mod coin;
use coin::Coin;

mod bonus;
use bonus::Bonus;

mod enemy;
use enemy::*;

mod player;
use player::*;

fn window_conf() -> Conf {
    let mut title = String::from("Pacman v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    Conf {
        window_title: title
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: resources::RES_WIDTH,
        window_height: resources::RES_HEIGHT,
        ..Default::default()
    }
}

pub enum GameState {
    Intro,
    InitLevel,
    Game,
    LevelCompleted,
    LevelFailed,
    GameOver,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Intro;
    let mut game = Game::new().await;
    let resources = Resources::new().await;
    let mut points: Vec<Point> = make_map_array(1);
    let mut small_coins: Vec<Coin> = Vec::new();
    let mut big_coins: Vec<Coin> = Vec::new();
    let mut player = Player::new().await;
    let mut bonuses: Vec<Bonus> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut animations: Vec<DieAnimation> = Vec::new();
    let mut eyes: Vec<Eyes> = Vec::new();

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Intro => {
                game.score = 0;
                game.lives = 2;
                game.lvl_num = 1;
                game_state = GameState::InitLevel;
            },
            GameState::InitLevel => {
                points.clear();
                big_coins.clear();
                small_coins.clear();
                bonuses.clear();
                enemies.clear();
                eyes.clear();
                points = make_map_array(game.lvl_num);
                player.x = PLAYER_START_X_POS;
                player.y = PLAYER_START_Y_POS;
                player.dir = PlayerDir::Left;
                game.scared_mode = false;
                game.last_bonus_was_at = get_time();

                // Load map objects
                for point in &mut points {
                    match point.value.as_str() {
                        "." => {
                            small_coins.push(
                                Coin::new(point.x as f32 * 50.0, point.y as f32 * 50.0, "small").await,
                            );
                        },
                        "O" => {
                            big_coins.push(
                                Coin::new(point.x as f32 * 50.0, point.y as f32 * 50.0, "big").await,
                            );
                        },
                        _ => {},
                    };
                }

                //load enemies
                let mut item_placed_successfully: bool;
                for _ in 1..=STARTING_AMOUNT_OF_ENEMY + game.lvl_num {
                    item_placed_successfully = false;
                    while !item_placed_successfully {
                        let x = rand::thread_rng().gen_range(0..=22);
                        let y = rand::thread_rng().gen_range(0..=10);
                        
                        if crate::levels::get_val(x,y, &points) == "s" {
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
            },
            GameState::Game => {
                draw_map(&points, &mut game);
                draw_score(resources.font,&game.score.to_string());
                player.draw_lives(game.lives);
                player.update(&points);
                
                for coin in &mut small_coins {
                    coin.draw();

                    if let Some(_i) = coin.rect.intersect(player.rect) {
                        coin.destroyed = true;
                        game.score += 10;
                        play_sound(resources.coin, PlaySoundParams {
                            looped: false,
                            volume: 0.4,
                        });
                    }
                }

                for coin in &mut big_coins {
                    coin.draw();

                    if let Some(_i) = coin.rect.intersect(player.rect) {
                        coin.destroyed = true;
                        game.scared_mode = true;
                        game.scared_mode_started_at = get_time();
                        game.score += 50;
                        play_sound(resources.big_coin, PlaySoundParams {
                            looped: false,
                            volume: 0.4,
                        });
                    }
                }

                // Play warning for scarred mode end
                if get_time() - game.scared_mode_started_at > 4.0 {
                    if !game.siren_played && game.scared_mode {
                        play_sound(resources.siren, PlaySoundParams {
                            looped: false,
                            volume: 0.7,
                        });
                        game.siren_played = true;
                    }
                }
                
                // End scarred mode
                if get_time() - game.scared_mode_started_at > 6.0 {
                    game.scared_mode = false;
                    game.siren_played = false;
                }

                // Generate some bonus
                if get_time() - game.last_bonus_was_at > 15.0 {
                    let mut item_placed_successfully: bool = false;
                    while !item_placed_successfully {
                        let x = rand::thread_rng().gen_range(0..=22);
                        let y = rand::thread_rng().gen_range(0..=10);
                        if crate::levels::get_val(x,y, &points) != "#" && 
                            crate::levels::get_val(x,y, &points) != "=" && 
                            crate::levels::get_val(x,y, &points) != "-" && 
                            crate::levels::get_val(x,y, &points) != "s" && 
                            crate::levels::get_val(x,y, &points) != "O" {
                            
                            bonuses.push(
                                Bonus::new(x as f32 * 50.0, y as f32 * 50.0).await,
                            );
                            game.last_bonus_was_at = get_time();
                            item_placed_successfully = true;
                        }
                    }
                }

                for bonus in &mut bonuses {
                    bonus.draw();

                    if get_time() - bonus.bonus_started_at > 6.0 {
                        bonus.destroyed = true;
                    }

                    if let Some(_i) = bonus.rect.intersect(player.rect) {
                        bonus.destroyed = true;
                        game.score += 150;
                        play_sound(resources.bonus, PlaySoundParams {
                            looped: false,
                            volume: 0.4,
                        });
                    }
                }

                for enemy in &mut enemies {
                    enemy.update(&points, &game);
                    enemy.scared_mode = game.scared_mode;
                    if let Some(_i) = enemy.rect.intersect(player.rect) {
                        enemy.destroyed = true;
                        if enemy.scared_mode {
                            play_sound(resources.eat_ghost, PlaySoundParams {
                                looped: false,
                                volume: 0.2,
                            });
                            game.score += 150;
                            eyes.push(
                                Eyes::new((enemy.x / 50.0).round() * 50.0 , (enemy.y / 50.0).round() * 50.0).await,
                            );
                        } else {
                            animations.push(
                                DieAnimation::new(player.x, player.y).await,
                            );
                            play_sound(resources.die, PlaySoundParams {
                                looped: false,
                                volume: 0.4,
                            });
                            game_state = GameState::LevelFailed;
                        }
                    }
                    enemy.draw();
                }

                for eye in &mut eyes {
                    eye.update(&points);
                    eye.draw();
                }

                if small_coins.len() == 0 {
                    game_state = GameState::LevelCompleted;
                }

                player.draw();
            },
            GameState::LevelCompleted => {
                draw_map(&points, &mut game);
                draw_score(resources.font,&game.score.to_string());
                player.draw_lives(game.lives);

                if game.lvl_num == 2 {
                    game.lvl_num = 0;
                }

                show_press_space_text(resources.font);

                if is_key_pressed(KeyCode::Space) {
                    game.lvl_num += 1;
                    player.x = PLAYER_START_X_POS;
                    player.y = PLAYER_START_Y_POS;
                    player.dir = PlayerDir::Left;
                    game_state = GameState::InitLevel;
                }
            },
            GameState::LevelFailed => {
                draw_map(&points, &mut game);

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
                        player.x = PLAYER_START_X_POS;
                        player.y = PLAYER_START_Y_POS;
                        player.dir = PlayerDir::Left;
                        game_state = GameState::Game;
                    } else {
                        game_state = GameState::GameOver;
                    }
                }
            },
            GameState::GameOver => {
                draw_map(&points, &mut game);

                show_press_space_text(resources.font);

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Intro;
                }
            },
        };

        // GC
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

        match bonuses.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                bonuses.remove(idx);
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

        match eyes.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                eyes.remove(idx);
            },
            None => {},
        };

        next_frame().await
    }
}