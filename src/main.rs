use macroquad::{prelude::*};

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
                    map.lvl_num += 1;
                    game_state = GameState::Game;
                }
            }

            GameState::Game => {
                map.draw();

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

                // DEBUG
                //println!("{} {}", player.x, player.y);

                for coin in &mut small_coins {
                    coin.draw();
                }

                for coin in &mut big_coins {
                    coin.draw();
                }

                player.draw();
            }
        }

        next_frame().await
    }
}