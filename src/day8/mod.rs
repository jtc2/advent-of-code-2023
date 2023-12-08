use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;

#[derive(Debug)]
pub enum Answer {
    Int(i64),
    String(String),
}

#[derive(Debug)]
struct Node {
    left_value: String,
    right_value: String,
}

fn parse_input(
    lines: impl Iterator<Item = Result<String, io::Error>>,
) -> (String, HashMap<String, Node>) {
    let mut node_map: HashMap<String, Node> = HashMap::new();
    let mut start_string: String = "".to_string();
    for line in lines {
        let read_line = line.unwrap();
        let mut split_line = read_line.split(" = ");
        let value = split_line.next().unwrap();
        if start_string == "" {
            start_string = value.clone().to_string();
        }
        let mut rest_line = split_line.next().unwrap().split(", ");
        let left_value = rest_line
            .next()
            .unwrap()
            .trim_start_matches("(")
            .to_string();
        let right_value = rest_line.next().unwrap().trim_end_matches(")").to_string();
        node_map.insert(
            value.to_string(),
            Node {
                left_value,
                right_value,
            },
        );
    }
    (start_string, node_map)
}

fn traverse_map(
    instructions: String,
    start_string: String,
    node_map: HashMap<String, Node>,
) -> i64 {
    let mut step_count = 0;
    let mut curr_string = start_string;
    println!("{:?}", instructions.chars());
    'outer: loop {
        for instruction in instructions.chars() {
            step_count += 1;

            if instruction == 'R' {
                curr_string = node_map
                    .get(&curr_string)
                    .unwrap()
                    .right_value
                    .clone()
                    .to_string();
            } else {
                curr_string = node_map
                    .get(&curr_string)
                    .unwrap()
                    .left_value
                    .clone()
                    .to_string();
            }
            println!("GOING TO: {:?}", curr_string);
        }
        println!("REDOING LOOP!");
        if curr_string == "ZZZ" {
            break 'outer;
        }
    }
    step_count
}

pub fn day8() -> Answer {
    let mut lines = read_file("./src/day8/input.txt");

    let instructions = lines.next().unwrap().unwrap();
    lines.next(); // skip blank line
    let (start_string, node_map) = parse_input(lines);
    println!("{:?}", node_map);
    let result = traverse_map(instructions, "AAA".to_string(), node_map);
    Answer::Int(result)
}

fn read_file(filename: &str) -> impl Iterator<Item = Result<String, io::Error>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().into_iter()
}
