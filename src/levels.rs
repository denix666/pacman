use macroquad::prelude::*;
use crate::{points::Point, game::Game};

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
            color: BROWN,
            ..Default::default()
        },
    );
}

// get and return value from the map
pub fn get_val(check_x: i32, check_y: i32, points: &Vec<Point>) -> String {
    let result = match points.iter().position(|x| x.x == check_x && x.y == check_y) {
        Some(idx) => points[idx].value.to_string(),
        _ => String::from("empty"),
    };
    return result
}

pub fn draw_map(points: &Vec<Point>, game: &mut Game) {
    for point in points {
        match point.value.as_str() {
            "#" => {
                draw_rectangle(point.x as f32 * 50.0, point.y as f32 * 50.0, 50.0, 50.0, DARKBROWN);
            },
            "-" => {
                draw_rectangle(point.x as f32 * 50.0, point.y as f32 * 50.0 + 20.0, 50.0, 5.0, WHITE);
                game.spawn_gate_x = point.x as f32 * 50.0;
                game.spawn_gate_y = point.y as f32 * 50.0;
            },
            _ => {},
        };
    }
}

pub fn make_map_array(lvl_num: i32) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    let map = match lvl_num {
        1 => vec![
            "=======================",
            "#######################",
            "#O...................O#",
            "#.#.#####.###.#####.#.#",
            "#.#.................#.#",
            "#.###.#.###-###.#.###.#",
            "#.#...#.#sssss#.#...#.#",
            "#.#.###.#sssss#.###.#.#",
            "#.......#sssss#.......#",
            "#.#####.#######.#####.#",
            "#.#.................#.#",
            "#.#.#.#.##.#.##.#.#.#.#",
            "#.#.#.#.##.#.##.#.#.#.#",
            "#O..#.............#..O#",
            "#######################",
        ],
        2 => vec![
            "=======================",
            "#######################",
            "#O...................O#",
            "#.#.#####.###.#####.#.#",
            "#.#.................#.#",
            "#.###.#.###-###.#.###.#",
            "#.#...#.#sssss#.#...#.#",
            "#.#.###.#sssss#.###.#.#",
            "#.......#sssss#.......#",
            "#.#####.#######.#####.#",
            "#.#.................#.#",
            "#.#.#.#.##.#.##.#.#.#.#",
            "#.#.#.#.##.#.##.#.#.#.#",
            "#O..#......#......#..O#",
            "#######################",
        ],
        _ => panic!("no such level"),
    };
    
    let mut mx: i32 = 0;
    let mut my: i32 = 0;
    for line in map {
        for c in line.chars() {
            points.push(
                Point::new(mx,my,c.to_string()),
            );
            mx += 1;
        }
        my += 1;
        mx = 0;
    }

    return points
}