use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

#[derive(Debug)]
pub enum Answer {
    Int(i32),
    String(String),
}

#[derive(Debug, Default)]
struct Game {
    red: i32,
    blue: i32,
    green: i32,
}

fn get_value_for_game(line: String) -> i32 {
    let mut line_split = line.split(": ");
    let game_id = line_split.next().unwrap();
    let game_num: i32 = game_id.split(" ").last().unwrap().parse().unwrap();

    for pull in line_split.last().unwrap().split("; ") {
        let mut current_pull = Game::default();

        for color in pull.split(", ") {
            let mut color_parts = color.split(" ");
            let count: i32 = color_parts.next().unwrap().parse().unwrap();
            let color = color_parts.next().unwrap();

            if color == "red" {
                current_pull.red = count;
            }
            if color == "blue" {
                current_pull.blue = count;
            }
            if color == "green" {
                current_pull.green = count;
            }
        }

        if current_pull.blue > MAX_BLUE
            || current_pull.red > MAX_RED
            || current_pull.green > MAX_GREEN
        {
            return 0;
        }
    }

    game_num
}

fn get_min_power_for_game(line: String) -> i32 {
    let line_split = line.split(": ");

    let mut min_green: i32 = 0;
    let mut min_blue: i32 = 0;
    let mut min_red: i32 = 0;

    for pull in line_split.last().unwrap().split("; ") {
        for color in pull.split(", ") {
            let mut color_parts = color.split(" ");
            let count: i32 = color_parts.next().unwrap().parse().unwrap();
            let color = color_parts.next().unwrap();

            if color == "red" && count > min_red {
                min_red = count;
            }
            if color == "blue" && count > min_blue {
                min_blue = count;
            }
            if color == "green" && count > min_green {
                min_green = count;
            }
        }
    }

    min_green * min_blue * min_red
}

pub fn day2() -> Answer {
    let lines = read_file("./src/day2/input.txt");
    let mut result: i32 = 0;
    for line in lines {
        result += get_min_power_for_game(line.unwrap());
    }

    Answer::Int(result)
}

fn read_file(filename: &str) -> impl Iterator<Item = Result<String, io::Error>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().into_iter()
}
