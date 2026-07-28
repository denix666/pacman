use macroquad::{prelude::*, audio::{PlaySoundParams, play_sound}};

mod resources;
use resources::*;

mod points;
use points::*;

mod die_animation;
use die_animation::*;

mod bonus_animation;
use bonus_animation::*;

mod levels;
use levels::*;

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
        window_title: title.to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: RES_WIDTH,
        window_height: RES_HEIGHT,
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

fn spawn_enemies(points: &[Point], game: &Game) -> Vec<Enemy> {
    let mut enemies = Vec::new();

    // Find spawn cells ("s") to place ghosts at fixed positions
    let mut spawn_cells: Vec<(f32, f32)> = Vec::new();
    for point in points {
        if point.value == "s" {
            spawn_cells.push((point.x as f32 * 50.0, point.y as f32 * 50.0));
        }
    }

    // Place ghosts in the house: one of each color up to amount_of_enemy
    // Spread them across available spawn cells
    let colors = [0, 1, 2, 3]; // red, blue, pink, green
    for i in 0..game.amount_of_enemy as usize {
        let color = colors[i % 4];
        let cell_idx = i % spawn_cells.len();
        let (x, y) = spawn_cells[cell_idx];
        enemies.push(Enemy::new(x, y, color));
    }

    enemies
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Intro;
    let mut game = Game::new();
    let resources = Resources::new().await;
    let mut points: Vec<Point> = make_map_array(1);
    let mut small_coins: Vec<Coin> = Vec::new();
    let mut big_coins: Vec<Coin> = Vec::new();
    let mut player = Player::new();
    let mut bonuses: Vec<Bonus> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut die_animations: Vec<DieAnimation> = Vec::new();
    let mut bonus_animations: Vec<BonusAnimation> = Vec::new();
    let mut last_release_time: f64 = 0.0;
    let mut ghosts_released: i32 = 0;

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Intro => {
                draw_texture(&resources.intro_texture, 0.0, 0.0, WHITE);
                show_press_space_text(&resources.font);

                if game.high_score > 0 {
                    draw_text_ex(
                        &format!("HIGH SCORE: {}", game.high_score),
                        390.0, 500.0,
                        TextParams {
                            font: Some(&resources.font),
                            font_size: 35,
                            color: YELLOW,
                            ..Default::default()
                        },
                    );
                }

                if is_key_pressed(KeyCode::Space) {
                    game.score = 0;
                    game.lives = 2;
                    game.lvl_num = 1;
                    game.next_life_at = 10000;
                    game.amount_of_enemy = STARTING_AMOUNT_OF_ENEMY;
                    game_state = GameState::InitLevel;
                }
            },
            GameState::InitLevel => {
                play_sound(&resources.beginning_snd, PlaySoundParams {
                    looped: false,
                    volume: 0.5,
                });
                points.clear();
                big_coins.clear();
                small_coins.clear();
                bonuses.clear();
                enemies.clear();
                points = make_map_array(game.lvl_num);
                player.x = PLAYER_START_X_POS;
                player.y = PLAYER_START_Y_POS;
                player.dir = PlayerDir::Left;
                game.scared_mode = false;
                game.last_bonus_was_at = get_time();
                last_release_time = get_time();
                ghosts_released = 0;

                for point in &points {
                    match point.value.as_str() {
                        "." => {
                            small_coins.push(Coin::new(point.x as f32 * 50.0, point.y as f32 * 50.0, false));
                        },
                        "O" => {
                            big_coins.push(Coin::new(point.x as f32 * 50.0, point.y as f32 * 50.0, true));
                        },
                        _ => {},
                    };
                }

                enemies = spawn_enemies(&points, &game);

                // Release first ghost immediately
                if !enemies.is_empty() {
                    enemies[0].release();
                    ghosts_released = 1;
                    last_release_time = get_time();
                }

                game_state = GameState::Game;
            },
            GameState::Game => {
                draw_map(&points, &mut game);
                draw_score(&resources.font, &game.score.to_string(), &game.high_score.to_string());
                player.draw_lives(game.lives, &resources);
                player.update(&points);

                for coin in &mut small_coins {
                    coin.draw(&resources);
                    if let Some(_i) = coin.rect.intersect(player.rect) {
                        coin.destroyed = true;
                        game.score += 10;
                        play_sound(&resources.coin_snd, PlaySoundParams {
                            looped: false,
                            volume: 0.4,
                        });
                    }
                }

                for coin in &mut big_coins {
                    coin.draw(&resources);
                    if let Some(_i) = coin.rect.intersect(player.rect) {
                        coin.destroyed = true;
                        game.scared_mode = true;
                        game.scared_mode_started_at = get_time();
                        game.score += 50;
                        play_sound(&resources.big_coin_snd, PlaySoundParams {
                            looped: false,
                            volume: 0.4,
                        });
                        for enemy in &mut enemies {
                            enemy.immune_to_scared = false;
                        }
                    }
                }

                // Scared mode warning
                if get_time() - game.scared_mode_started_at > 4.0 {
                    if !game.siren_played && game.scared_mode {
                        play_sound(&resources.siren_snd, PlaySoundParams {
                            looped: false,
                            volume: 0.7,
                        });
                        game.siren_played = true;
                    }
                }

                // End scared mode
                if get_time() - game.scared_mode_started_at > 6.0 {
                    game.scared_mode = false;
                    game.siren_played = false;
                }

                // Release ghosts one by one every 4 seconds
                if ghosts_released < enemies.len() as i32 && get_time() - last_release_time > 4.0 {
                    enemies[ghosts_released as usize].release();
                    ghosts_released += 1;
                    last_release_time = get_time();
                }

                // Generate bonus
                if get_time() - game.last_bonus_was_at > 15.0 {
                    let mut placed = false;
                    let mut attempts = 0;
                    while !placed && attempts < 100 {
                        let x = ::rand::random_range(0..=22);
                        let y = ::rand::random_range(0..=10);
                        let val = get_val(x, y, &points);
                        if val != "#" && val != "=" && val != "-" && val != "s" && val != "O" && val != "T" {
                            bonuses.push(Bonus::new(x as f32 * 50.0, y as f32 * 50.0));
                            game.last_bonus_was_at = get_time();
                            placed = true;
                        }
                        attempts += 1;
                    }
                }

                for bonus in &mut bonuses {
                    bonus.draw(&resources);
                    if get_time() - bonus.bonus_started_at > 6.0 {
                        bonus.destroyed = true;
                    }
                    if let Some(_i) = bonus.rect.intersect(player.rect) {
                        bonus.destroyed = true;
                        game.score += 100;
                        play_sound(&resources.bonus_snd, PlaySoundParams {
                            looped: false,
                            volume: 0.4,
                        });
                        bonus_animations.push(BonusAnimation::new(bonus.x, bonus.y));
                    }
                }

                for animation in &mut bonus_animations {
                    animation.draw(&resources);
                }

                for enemy in &mut enemies {
                    if enemy.state == GhostState::Active && !enemy.immune_to_scared {
                        enemy.scared_mode = game.scared_mode;
                    } else if enemy.state != GhostState::Active {
                        enemy.scared_mode = false;
                    }

                    enemy.update(&points, &game, player.x, player.y, &player.dir);

                    // Collision
                    if enemy.is_collidable() {
                        if let Some(_i) = enemy.rect.intersect(player.rect) {
                            if enemy.scared_mode {
                                play_sound(&resources.eat_ghost_snd, PlaySoundParams {
                                    looped: false,
                                    volume: 0.2,
                                });
                                game.score += 150;
                                enemy.become_eyes();
                            } else {
                                die_animations.push(DieAnimation::new(player.x, player.y));
                                play_sound(&resources.die_snd, PlaySoundParams {
                                    looped: false,
                                    volume: 0.2,
                                });
                                game_state = GameState::LevelFailed;
                            }
                        }
                    }

                    enemy.draw(&resources);
                }

                if game.score >= game.next_life_at {
                    game.lives += 1;
                    game.next_life_at += 10000;
                    play_sound(&resources.new_live_snd, PlaySoundParams {
                        looped: false,
                        volume: 0.5,
                    });
                }

                game.update_high_score();

                if small_coins.is_empty() {
                    game_state = GameState::LevelCompleted;
                }

                player.draw(&resources);
            },
            GameState::LevelCompleted => {
                draw_map(&points, &mut game);
                draw_score(&resources.font, &game.score.to_string(), &game.high_score.to_string());
                player.draw_lives(game.lives, &resources);

                if game.lvl_num == 3 {
                    game.lvl_num = 0;
                }

                show_press_space_text(&resources.font);

                if is_key_pressed(KeyCode::Space) {
                    game.lvl_num += 1;
                    if game.amount_of_enemy < 6 {
                        game.amount_of_enemy += 1;
                    }
                    player.x = PLAYER_START_X_POS;
                    player.y = PLAYER_START_Y_POS;
                    player.dir = PlayerDir::Left;
                    game_state = GameState::InitLevel;
                }
            },
            GameState::LevelFailed => {
                draw_map(&points, &mut game);

                for coin in &mut small_coins {
                    coin.draw(&resources);
                }
                for coin in &mut big_coins {
                    coin.draw(&resources);
                }
                for en in &mut enemies {
                    en.draw(&resources);
                }

                player.draw_lives(game.lives, &resources);
                draw_score(&resources.font, &game.score.to_string(), &game.high_score.to_string());

                for animation in &mut die_animations {
                    animation.draw(&resources);
                }

                show_press_space_text(&resources.font);

                if die_animations.is_empty() && is_key_pressed(KeyCode::Space) {
                    if game.lives > 0 {
                        game.lives -= 1;
                        player.x = PLAYER_START_X_POS;
                        player.y = PLAYER_START_Y_POS;
                        player.dir = PlayerDir::Left;

                        enemies = spawn_enemies(&points, &game);
                        last_release_time = get_time();
                        ghosts_released = 0;
                        if !enemies.is_empty() {
                            enemies[0].release();
                            ghosts_released = 1;
                            last_release_time = get_time();
                        }

                        game_state = GameState::Game;
                    } else {
                        game.update_high_score();
                        game_state = GameState::GameOver;
                    }
                }
            },
            GameState::GameOver => {
                draw_map(&points, &mut game);

                draw_text_ex(
                    "GAME OVER",
                    380.0, 400.0,
                    TextParams {
                        font: Some(&resources.font),
                        font_size: 60,
                        color: RED,
                        ..Default::default()
                    },
                );

                draw_text_ex(
                    &format!("HIGH SCORE: {}", game.high_score),
                    390.0, 470.0,
                    TextParams {
                        font: Some(&resources.font),
                        font_size: 40,
                        color: YELLOW,
                        ..Default::default()
                    },
                );

                show_press_space_text(&resources.font);

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Intro;
                }
            },
        };

        small_coins.retain(|x| !x.destroyed);
        big_coins.retain(|x| !x.destroyed);
        bonuses.retain(|x| !x.destroyed);
        die_animations.retain(|x| !x.destroyed);
        bonus_animations.retain(|x| !x.destroyed);

        next_frame().await
    }
}
