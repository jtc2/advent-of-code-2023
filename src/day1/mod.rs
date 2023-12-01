use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub enum Answer {
    Int(i64),
    String(String),
}

fn get_substring(line: &String, idx: usize, len: i8, reversed: bool) -> String {
    let substr = &line[idx..cmp::min(idx + len as usize, line.len())];
    if reversed {
        return substr.chars().rev().collect();
    }
    substr.to_string()
}

fn is_written_number(line: &String, idx: usize, reversed: bool) -> i64 {
    let mut value = match get_substring(line, idx, 3, reversed).as_str() {
        "one" => 1,
        "two" => 2,
        "six" => 6,
        _ => -1,
    };
    if value != -1 {
        return value;
    }

    value = match get_substring(line, idx, 4, reversed).as_str() {
        "zero" => 0,
        "four" => 4,
        "five" => 5,
        "nine" => 9,
        _ => -1,
    };
    if value != -1 {
        return value;
    }

    return match get_substring(line, idx, 5, reversed).as_str() {
        "three" => 3,
        "seven" => 7,
        "eight" => 8,
        _ => -1,
    };
}

fn get_first_digit(line: &String, reversed: bool) -> i64 {
    for (idx, character) in line.chars().enumerate() {
        if character.is_digit(10) {
            return character.to_digit(10).unwrap() as i64;
        }
        let digit = is_written_number(line, idx, reversed);
        if digit != -1 {
            return digit;
        }
    }
    0
}

fn get_value_for_line(line: String) -> i64 {
    let first_val = get_first_digit(&line, false);
    let last_val = get_first_digit(&line.chars().rev().collect(), true);

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

fn read_file(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
