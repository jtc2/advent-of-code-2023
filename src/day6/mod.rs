use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;

#[derive(Debug)]
pub enum Answer {
    Int(i64),
    String(String),
}

#[derive(Debug)]
struct Race {
    time: i64,
    record_distance: i64,
}

impl Race {
    fn calculate_distance(&self, start_time: i64) -> i64 {
        let time_spent_moving = self.time - start_time;
        let speed = start_time;
        return speed * time_spent_moving;
    }

    fn get_num_better_times(&self) -> i64 {
        let mut count_better_times = 0;
        for start_time in 0..self.time {
            if self.calculate_distance(start_time) > self.record_distance {
                count_better_times += 1
            }
        }
        count_better_times
    }
}

fn parse_input(mut lines: impl Iterator<Item = Result<String, io::Error>>) -> Vec<Race> {
    let time_line = lines.next().unwrap().unwrap();
    let distance_line = lines.next().unwrap().unwrap();

    let starting_regex = Regex::new(r":\s+").unwrap();
    let whitespace_regex = Regex::new(r"\s+").unwrap();

    let time_line_num_strs = starting_regex.split(&time_line).last().unwrap();
    let time_line_nums = whitespace_regex
        .split(time_line_num_strs)
        .map(|val| val.parse().unwrap())
        .collect::<Vec<i64>>();

    let distance_line_num_strs = starting_regex.split(&distance_line).last().unwrap();
    let distance_line_nums = whitespace_regex
        .split(distance_line_num_strs)
        .map(|val| val.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut races: Vec<Race> = vec![];
    for (time, distance) in zip(time_line_nums, distance_line_nums) {
        races.push(Race {
            time: time,
            record_distance: distance,
        })
    }

    races
}

pub fn day6() -> Answer {
    let lines = read_file("./src/day6/input.txt");
    let races = parse_input(lines);

    let mut result: i64 = races[0].get_num_better_times();

    Answer::Int(result)
}

fn read_file(filename: &str) -> impl Iterator<Item = Result<String, io::Error>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().into_iter()
}
