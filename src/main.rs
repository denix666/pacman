use macroquad::{prelude::*, audio::{PlaySoundParams, play_sound}};
extern crate rand;
use rand::{Rng};

pub const RES_WIDTH: i32 = 1150;
pub const RES_HEIGHT: i32 = 750;
pub const STARTING_AMOUNT_OF_ENEMY: i32 = 2;

mod res;
use res::*;

mod messages;
use messages::*;

mod points;
use points::*;

mod game;
use game::*;

mod coin;
use coin::*;

mod player;
use player::*;

mod enemy;
use enemy::*;

mod eyes;
use eyes::*;

mod die_animation;
use die_animation::*;

pub enum GameState {
    Game,
    Intro,
    InitLevel,
    LevelCompleted,
    LevelFailed,
    GameCompleted,
    GameOver,
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

// Get value from the map
fn get_val(check_x: u32, check_y: u32, points: &Vec<Point>) -> String {
    
    let ret = match points.iter().position(|x| x.x == check_x && x.y == check_y) {
        Some(idx) => points[idx].value.to_string(),
        _ => String::from("empty"),
    };
    ret
}

// Draw the map
fn draw_map(map_color: Color, points: &Vec<Point>) {
    for point in points {
        match point.value.as_str() {
            "X" => {
                draw_rectangle(point.x as f32 * 50.0, point.y as f32 * 50.0, 50.0, 50.0, map_color);
            },
            "-" => {
                draw_rectangle(point.x as f32 * 50.0, point.y as f32 * 50.0, 50.0, 5.0, WHITE);
            },
            _ => {},
        };
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Intro;
    let resources = Resources::new().await;
    let mut game = Game::new().await;
    let mut points: Vec<Point> = Vec::new();
    let mut small_coins: Vec<Coin> = Vec::new();
    let mut big_coins: Vec<Coin> = Vec::new();
    let mut player = Player::new().await;
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut eyes: Vec<Eyes> = Vec::new();
    let mut animations: Vec<DieAnimation> = Vec::new();

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
            },
            GameState::InitLevel => {
                big_coins.clear();
                small_coins.clear();
                points.clear();
                eyes.clear();
                enemies.clear();
                game.scared_mode = false;
                player.x = 550.0;
                player.y = 650.0;
                player.dir = PlayerDir::Left;

                /////////////////////
                //  X  wall
                //  O  big coin
                //  .  small coin
                //  s  spawn place
                //  -  spawn door
                /////////////////////

                let lvl1 = vec![
                    "=======================",
                    "XXXXXXXXXXXXXXXXXXXXXXX",
                    "XO...................OX",
                    "X.X.XXXXX.XXX.XXXXX.X.X",
                    "X.X.................X.X",
                    "X.XXX.X.XXX-XXX.X.XXX.X",
                    "X.X...X.XsssssX.X...X.X",
                    "X.X.XXX.XsssssX.XXX.X.X",
                    "X.......XsssssX.......X",
                    "X.XXXXX.XXXXXXX.XXXXX.X",
                    "X.X.................X.X",
                    "X.X.X.X.XX.X.XX.X.X.X.X",
                    "X.X.X.X.XX.X.XX.X.X.X.X",
                    "XO..X.............X..OX",
                    "XXXXXXXXXXXXXXXXXXXXXXX",
                ];

                let lvl2 = vec![
                    "=======================",
                    "XXXXXXXXXXXXXXXXXXXXXXX",
                    "XO...................OX",
                    "X.X.XXXXXXXXXXXXXXX.X.X",
                    "X.X.................X.X",
                    "X.XXX.X.XXX-XXX.X.XXX.X",
                    "X.X...X.XsssssX.X...X.X",
                    "X.X.XXX.XXXXXXX.XXX.X.X",
                    "X.....................X",
                    "X.XXXXX.X.X.X.X.XXXXX.X",
                    "X.....X.X.X.X.X.X.....X",
                    "X.XXX.X.X.X.X.X.X.XXX.X",
                    "X.XXX.X.X.X.X.X.X.XXX.X",
                    "XO...................OX",
                    "XXXXXXXXXXXXXXXXXXXXXXX",
                ];

                let lvl;
                match game.level_num {
                    1  => {
                        lvl = lvl1;
                        game.map_color = BLUE;
                    },
                    2  => {
                        lvl = lvl2;
                        game.map_color = GREEN;
                    },
                    _ => {
                        panic!("no such level");
                    }
                };

                // Read map
                let mut mx: u32 = 0;
                let mut my: u32 = 0;
                for line in lvl {
                    for c in line.chars() {
                        points.push(
                            Point::new(mx,my,c.to_string()),
                        );
                        mx += 1;
                    }
                    my += 1;
                    mx = 0;
                }

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
                for _ in 1..=STARTING_AMOUNT_OF_ENEMY + game.level_num {
                    item_placed_successfully = false;
                    while !item_placed_successfully {
                        let x = rand::thread_rng().gen_range(0..=22);
                        let y = rand::thread_rng().gen_range(0..=10);
                        
                        if get_val(x,y, &points) != "X" && 
                            get_val(x,y, &points) != "=" && 
                            get_val(x,y, &points) != "-" {
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
            GameState::LevelCompleted => {
                draw_map(game.map_color, &points);

                if game.level_num == 2 {
                    game_state = GameState::GameCompleted;
                }

                show_level_completed_text(resources.font);

                if is_key_pressed(KeyCode::Space) {
                    game.level_num += 1;
                    player.x = 550.0;
                    player.y = 650.0;
                    player.dir = PlayerDir::Left;
                    game_state = GameState::InitLevel;
                }
            },
            GameState::GameOver => {
                draw_map(game.map_color, &points);

                show_game_over_text(resources.font);

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Intro;
                }
            },
            GameState::GameCompleted => {
                draw_map(game.map_color, &points);

                show_game_completed_text(resources.font);

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Intro;
                }
            },
            GameState::LevelFailed => {
                draw_map(game.map_color, &points);

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
                        game_state = GameState::GameOver;
                    }
                }
            }, 
            GameState::Game => {
                draw_map(game.map_color, &points);

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
                        game.scared_mode = true;
                        game.scared_mode_started_at = get_time();
                        game.score += 50;
                        play_sound(resources.bonus, PlaySoundParams {
                            looped: false,
                            volume: 0.2,
                        });
                    }
                }

                player.update(&points);
                player.draw();

                draw_score(resources.font,&game.score.to_string());
                player.draw_lives(game.lives);

                for enemy in &mut enemies {
                    enemy.update(&points);
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
                            game_state = GameState::LevelFailed;
                        }
                    }
                    enemy.draw();
                }

                for eye in &mut eyes {
                    eye.update(&points);
                    eye.draw();
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
                    game.siren_played = false;
                }

                if small_coins.len() == 0 {
                    game_state = GameState::LevelCompleted;
                }
            },
        }

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

        match eyes.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                eyes.remove(idx);
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