use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub enum Answer {
    Int(i64),
    String(String),
}

#[derive(Debug)]
struct LotteryTicket {
    winning_numbers: HashSet<i32>,
    played_numbers: HashSet<i32>,
}

impl LotteryTicket {
    fn parse_ticket(line: String) -> LotteryTicket {
        let first_regex = Regex::new(r":\s+").unwrap();
        let parts = first_regex.split(&line);
        let mid_regex = Regex::new(r"\s+\|\s+").unwrap();
        let mut numbers = mid_regex.split(parts.last().unwrap());
        // println!("{:?}", numbers.next());

        LotteryTicket {
            winning_numbers: HashSet::from_iter(
                Regex::new(r"\s+")
                    .unwrap()
                    .split(numbers.next().unwrap())
                    .map(|val| val.parse().unwrap())
                    .collect::<Vec<i32>>(),
            ),
            played_numbers: HashSet::from_iter(
                Regex::new(r"\s+")
                    .unwrap()
                    .split(numbers.next().unwrap())
                    .map(|val| val.parse().unwrap())
                    .collect::<Vec<i32>>(),
            ),
        }
    }

    fn calculate_wins(&self) -> i64 {
        let mut count_wins = 0;

        for played_number in &self.played_numbers {
            if self.winning_numbers.contains(played_number) {
                count_wins += 1;
            }
        }

        count_wins
    }

    fn calculate_value(&self) -> i64 {
        let wins = self.calculate_wins();
        if wins == 0 {
            return 0;
        }

        2i64.pow(wins as u32 - 1)
    }
}

pub fn day4() -> Answer {
    let lines = read_file("./src/day4/input.txt");
    let mut result: i64 = 0;
    let mut count_tickets: HashMap<i32, i32> = HashMap::new();
    // Game 1 only ever has 1 ticket
    for (index, line) in lines.enumerate() {
        // First, add our original copy of the card
        let curr_ticket = count_tickets.entry(index as i32).or_default();
        *curr_ticket += 1;
        let curr_ticket_count = *curr_ticket;

        let lottery_ticket = LotteryTicket::parse_ticket(line.unwrap());
        // result += lottery_ticket.calculate_value();
        let num_wins = lottery_ticket.calculate_wins();
        let mut next_index: i32 = index as i32 + 1;
        for loop_var in 0..num_wins {
            let value = count_tickets
                .entry(next_index + loop_var as i32)
                .or_default();
            *value += curr_ticket_count;
        }
        // println!("{:#?}", count_tickets);
    }

    Answer::Int(
        count_tickets
            .into_values()
            .reduce(|acc, e| acc + e)
            .unwrap() as i64,
    )
}

fn read_file(filename: &str) -> impl Iterator<Item = Result<String, io::Error>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().into_iter()
}
