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

// constants to represent hand types. Lower value = worse hand
const HIGH_CARD: u8 = 1;
const ONE_PAIR: u8 = 2;
const TWO_PAIR: u8 = 3;
const THREE_KIND: u8 = 4;
const FULL_HOUSE: u8 = 5;
const FOUR_KIND: u8 = 6;
const FIVE_KIND: u8 = 7;

const CHAR_COMP_ARR: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const CHAR_COMP_ARR_PART_2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i64,
    value: u8,
}

impl Hand {
    fn get_value(cards: &str) -> u8 {
        let mut char_to_count: HashMap<char, u8> = HashMap::new();
        for char in cards.chars() {
            let value = char_to_count.entry(char).or_default();
            *value += 1;
        }

        // Determine the hand value based off the char count map
        let num_keys = char_to_count.keys().len();
        if num_keys == 1 {
            return FIVE_KIND;
        } else if num_keys == 2 {
            if char_to_count.values().into_iter().any(|val| *val == 4) {
                return FOUR_KIND;
            }
            return FULL_HOUSE;
        } else if num_keys == 3 {
            if char_to_count.values().into_iter().any(|val| *val == 3) {
                return THREE_KIND;
            }
            return TWO_PAIR;
        } else if num_keys == 4 {
            return ONE_PAIR;
        } else if num_keys == 5 {
            return HIGH_CARD;
        }
        0
    }

    fn get_value_with_jokers(cards: &str) -> u8 {
        let mut char_to_count: HashMap<char, u8> = HashMap::new();
        let mut joker_count = 0;
        for char in cards.chars() {
            if char == 'J' {
                joker_count += 1;
            } else {
                let value = char_to_count.entry(char).or_default();
                *value += 1;
            }
        }

        // Determine the hand value based off the char count map
        let num_keys = char_to_count.keys().len();
        if joker_count == 0 {
            if num_keys == 1 {
                return FIVE_KIND;
            } else if num_keys == 2 {
                if char_to_count.values().into_iter().any(|val| *val == 4) {
                    return FOUR_KIND;
                }
                return FULL_HOUSE;
            } else if num_keys == 3 {
                if char_to_count.values().into_iter().any(|val| *val == 3) {
                    return THREE_KIND;
                }
                return TWO_PAIR;
            } else if num_keys == 4 {
                return ONE_PAIR;
            } else if num_keys == 5 {
                return HIGH_CARD;
            }
        } else if joker_count == 1 {
            if num_keys == 1 {
                return FIVE_KIND;
            } else if num_keys == 2 {
                // can be 2-2 or 3-1
                if char_to_count.values().into_iter().all(|val| *val == 2) {
                    return FULL_HOUSE;
                }
                return FOUR_KIND;
            } else if num_keys == 3 {
                // must be 2-1-1
                return THREE_KIND;
            } else {
                // 1-1-1-1
                return ONE_PAIR;
            }
        } else if joker_count == 2 {
            if num_keys == 1 {
                return FIVE_KIND;
            } else if num_keys == 2 {
                // 2-1
                return FOUR_KIND;
            } else {
                return THREE_KIND;
            }
        } else if joker_count == 3 {
            if num_keys == 2 {
                return FOUR_KIND;
            } else {
                return FIVE_KIND;
            }
        } else {
            return FIVE_KIND;
        }
        0
    }

    fn create(cards: &str, bid: i64) -> Self {
        Self {
            cards: cards.to_string(),
            bid,
            value: Hand::get_value_with_jokers(cards),
        }
    }

    fn compare(self: &Hand, other: &Hand) -> Ordering {
        // same cards => equal hands
        if self.cards == other.cards {
            return Ordering::Equal;
        }

        // different values => sort by value
        if self.value < other.value {
            return Ordering::Less;
        } else if self.value > other.value {
            return Ordering::Greater;
        } else {
            for (self_char, other_char) in zip(self.cards.chars(), other.cards.chars()) {
                if self_char == other_char {
                    continue;
                }

                if CHAR_COMP_ARR_PART_2
                    .iter()
                    .position(|val| *val == self_char)
                    < CHAR_COMP_ARR_PART_2
                        .iter()
                        .position(|val| *val == other_char)
                {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
            // same strings, return equal
            return Ordering::Equal;
        }

        // default case
        Ordering::Equal
    }
}

fn parse_input(lines: impl Iterator<Item = Result<String, io::Error>>) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];
    for line in lines {
        let read_line = line.unwrap();
        let mut split_line = read_line.split(" ");
        hands.push(Hand::create(
            split_line.next().unwrap(),
            split_line.next().unwrap().parse().unwrap(),
        ))
    }
    hands
}

pub fn day7() -> Answer {
    let lines = read_file("./src/day7/input.txt");

    let mut hands = parse_input(lines);
    hands.sort_by(|a, b| Hand::compare(a, b));
    println!("{:?}", hands);
    let mut result = 0;
    for (index, hand) in hands.into_iter().enumerate() {
        result += hand.bid * (index as i64 + 1);
    }

    Answer::Int(result)
}

fn read_file(filename: &str) -> impl Iterator<Item = Result<String, io::Error>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().into_iter()
}
