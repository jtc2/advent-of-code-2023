use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub enum Answer {
    Int(i64),
    String(String),
}

fn is_written_number(line: &str, idx: usize) -> i64 {
    match &line[idx..cmp::min(idx + 3, line.len())] {
        "one" | "eno" => return 1,
        "two" | "owt" => return 2,
        "six" | "xis" => return 6,
        _ => -1,
    };

    match &line[idx..cmp::min(idx + 4, line.len())] {
        "zero" | "orez" => return 0,
        "four" | "ruof" => return 4,
        "five" | "evif" => return 5,
        "nine" | "enin" => return 9,
        _ => -1,
    };

    return match &line[idx..cmp::min(idx + 5, line.len())] {
        "three" | "eerht" => 3,
        "seven" | "neves" => 7,
        "eight" | "thgie" => 8,
        _ => -1,
    };
}

fn get_first_digit(line: &str) -> i64 {
    for (idx, character) in line.chars().enumerate() {
        if character.is_digit(10) {
            return character.to_digit(10).unwrap() as i64;
        }
        let digit = is_written_number(line, idx);
        if digit != -1 {
            return digit;
        }
    }
    0
}

fn get_value_for_line(line: String) -> i64 {
    let first_val = get_first_digit(&line);
    let last_val = get_first_digit(&line.chars().rev().collect::<String>());

    first_val * 10 + last_val
}

pub fn day1() -> Answer {
    let lines = read_file("./src/day1/input.txt");
    let mut result: i64 = 0;
    for line in lines {
        result += get_value_for_line(line.unwrap());
    }

    Answer::Int(result)
}

fn read_file(filename: &str) -> impl Iterator<Item = Result<String, io::Error>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().into_iter()
}
