use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

const SEED_TO_SOIL_IDX: usize = 0;
const SOIL_TO_FERTILIZER_IDX: usize = 1;
const FERTILIZER_TO_WATER_IDX: usize = 2;
const WATER_TO_LIGHT_IDX: usize = 3;
const LIGHT_TO_TEMP_IDX: usize = 4;
const TEMP_TO_HUMIDITY_IDX: usize = 5;
const HUMIDITY_TO_LOCATION_IDX: usize = 6;

#[derive(Debug)]
pub enum Answer {
    Int(i64),
    String(String),
}

#[derive(Clone, Copy, Debug, Default)]
struct InputRange {
    dest_start: i64,
    source_start: i64,
    range_length: i64,
    current_iter_value: i64,
}

impl Iterator for InputRange {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_iter_value == -1 {
            return None;
        }

        if self.current_iter_value == 0 {
            self.current_iter_value = self.dest_start - 1;
        }

        self.current_iter_value += 1;
        if self.current_iter_value > self.dest_start + self.range_length {
            return None;
        }

        Some(self.current_iter_value)
    }
}

impl InputRange {
    fn get_output_value(&self, input_value: i64) -> i64 {
        // If in source range, output dest value
        if (input_value >= self.source_start)
            && (input_value < self.source_start + self.range_length)
        {
            return input_value - self.source_start + self.dest_start;
        }

        // Outside input range, return input value
        input_value
    }

    fn get_input_value_from_output(&self, output_value: i64) -> i64 {
        if (output_value >= self.dest_start) && (output_value < self.dest_start + self.range_length)
        {
            return output_value - self.dest_start + self.source_start;
        }

        output_value
    }

    fn get_input_value_from_output_if_exists(&self, output_value: i64) -> i64 {
        if (output_value >= self.dest_start) && (output_value < self.dest_start + self.range_length)
        {
            return output_value - self.dest_start + self.source_start;
        }

        -1
    }
}

#[derive(Clone, Debug, Default)]
struct Ranges {
    ranges: Vec<InputRange>,
}

impl Ranges {
    fn get_final_output_value(&self, input_value: i64) -> i64 {
        for range in self.ranges.iter() {
            let range_output_value = range.get_output_value(input_value);
            if range_output_value != input_value {
                return range_output_value;
            }
        }

        input_value
    }

    fn get_original_input_value(&self, output_value: i64) -> i64 {
        for range in self.ranges.iter() {
            let range_input_value = range.get_input_value_from_output(output_value);
            if range_input_value != output_value {
                return range_input_value;
            }
        }

        output_value
    }

    fn get_original_input_value_if_exists(&self, output_value: i64) -> i64 {
        for range in self.ranges.iter() {
            let range_input_value = range.get_input_value_from_output_if_exists(output_value);
            // println!(
            //     "OUTPUT: {:?}, self: {:?}, RANGE INPUT: {:?}",
            //     output_value, self, range_input_value
            // );
            if range_input_value != -1 {
                return range_input_value;
            }
        }

        -1
    }
}

fn parse_input_seeds_part_1(line: String) -> HashSet<i64> {
    let mut starting_seeds: HashSet<i64> = HashSet::new();
    let mut seed_nums_str_list = line.split(": ").last().unwrap().split(" ");
    // Individual seeds
    for seed_num in seed_nums_str_list {
        starting_seeds.insert(seed_num.parse().unwrap());
    }
    starting_seeds
}

fn parse_input_seeds_part_2(line: String) -> Ranges {
    // let mut starting_seeds: Vec<i64> = vec![];
    // let mut seed_nums_str_list = line.split(": ").last().unwrap().split(" ");

    // let mut start_range: i64 = seed_nums_str_list.next().unwrap_or("-1").parse().unwrap();
    // while start_range != -1 {
    //     let length: i64 = seed_nums_str_list.next().unwrap_or("-1").parse().unwrap();

    //     for offset in 0..length {
    //         starting_seeds.push(start_range + offset);
    //     }
    //     // let curr_range = InputRange {
    //     //     dest_start: start_range,
    //     //     source_start: start_range,
    //     //     range_length: length,
    //     //     current_iter_value: 0,
    //     // };
    //     // seed_ranges.ranges.push(curr_range);
    //     start_range = seed_nums_str_list.next().unwrap_or("-1").parse().unwrap()
    // }
    // starting_seeds

    let mut seed_ranges = Ranges::default();
    let mut seed_nums_str_list = line.split(": ").last().unwrap().split(" ");
    let mut start_range: i64 = seed_nums_str_list.next().unwrap_or("-1").parse().unwrap();
    while start_range != -1 {
        let length: i64 = seed_nums_str_list.next().unwrap_or("-1").parse().unwrap();

        let curr_range = InputRange {
            dest_start: start_range,
            source_start: start_range,
            range_length: length,
            current_iter_value: 0,
        };

        seed_ranges.ranges.push(curr_range);
        start_range = seed_nums_str_list.next().unwrap_or("-1").parse().unwrap()
    }

    seed_ranges
}

fn parse_input(mut lines: impl Iterator<Item = Result<String, io::Error>>) -> Vec<Ranges> {
    // Initialize all needed data stores
    let mut range_list = vec![
        Ranges::default(),
        Ranges::default(),
        Ranges::default(),
        Ranges::default(),
        Ranges::default(),
        Ranges::default(),
        Ranges::default(),
    ];

    // Handle all other lines, storing current mapping to know what range to set
    lines.next(); // skip empty line

    // println!("{:?}", range_list[0].ranges);

    // Parse Input
    let mut curr_index = 0;
    for line in lines {
        let curr_line = line.unwrap();
        match curr_line.as_str() {
            "" => continue,
            "seed-to-soil map:" => curr_index = SEED_TO_SOIL_IDX,
            "soil-to-fertilizer map:" => curr_index = SOIL_TO_FERTILIZER_IDX,
            "fertilizer-to-water map:" => curr_index = FERTILIZER_TO_WATER_IDX,
            "water-to-light map:" => curr_index = WATER_TO_LIGHT_IDX,
            "light-to-temperature map:" => curr_index = LIGHT_TO_TEMP_IDX,
            "temperature-to-humidity map:" => curr_index = TEMP_TO_HUMIDITY_IDX,
            "humidity-to-location map:" => curr_index = HUMIDITY_TO_LOCATION_IDX,
            _ => {
                // At this point, have to insert a range
                let mut split_parts = curr_line.split(" ");
                range_list[curr_index].ranges.push(InputRange {
                    dest_start: split_parts.next().unwrap().parse().unwrap(),
                    source_start: split_parts.next().unwrap().parse().unwrap(),
                    range_length: split_parts.next().unwrap().parse().unwrap(),
                    current_iter_value: 0,
                });
            }
        }
    }

    // println!("{:#?}", starting_seeds);
    // println!("{:#?}", range_list);

    range_list
}

pub fn handle_part_1(mut lines: impl Iterator<Item = Result<String, io::Error>>) -> i64 {
    let starting_seeds = parse_input_seeds_part_1(lines.next().unwrap().unwrap());
    let ranges = parse_input(lines);

    // Map starting seeds to soil values
    let soil_values: Vec<i64> = starting_seeds
        .into_iter()
        .map(|seed| {
            ranges
                .get(SEED_TO_SOIL_IDX)
                .unwrap()
                .get_final_output_value(seed)
        })
        .collect();

    let fertilizer_values: Vec<i64> = soil_values
        .into_iter()
        .map(|soil| {
            ranges
                .get(SOIL_TO_FERTILIZER_IDX)
                .unwrap()
                .get_final_output_value(soil)
        })
        .collect();

    let water_values: Vec<i64> = fertilizer_values
        .into_iter()
        .map(|fertilizer| {
            ranges
                .get(FERTILIZER_TO_WATER_IDX)
                .unwrap()
                .get_final_output_value(fertilizer)
        })
        .collect();

    let light_values: Vec<i64> = water_values
        .into_iter()
        .map(|light| {
            ranges
                .get(WATER_TO_LIGHT_IDX)
                .unwrap()
                .get_final_output_value(light)
        })
        .collect();

    let temp_values: Vec<i64> = light_values
        .into_iter()
        .map(|light| {
            ranges
                .get(LIGHT_TO_TEMP_IDX)
                .unwrap()
                .get_final_output_value(light)
        })
        .collect();

    let humidity_values: Vec<i64> = temp_values
        .into_iter()
        .map(|temp| {
            ranges
                .get(TEMP_TO_HUMIDITY_IDX)
                .unwrap()
                .get_final_output_value(temp)
        })
        .collect();

    let location_values: Vec<i64> = humidity_values
        .into_iter()
        .map(|humidity| {
            ranges
                .get(HUMIDITY_TO_LOCATION_IDX)
                .unwrap()
                .get_final_output_value(humidity)
        })
        .collect();

    location_values.into_iter().min().unwrap()
}

pub fn handle_part_2(mut lines: impl Iterator<Item = Result<String, io::Error>>) -> i64 {
    let starting_seed_range = parse_input_seeds_part_2(lines.next().unwrap().unwrap());
    let ranges = parse_input(lines);
    println!("HERE I AM!");
    let mut curr_loc = 0;
    loop {
        if curr_loc % 10000 == 0 {
            println!("IN LOOP, idx: {:?}", curr_loc);
        }
        let humidity_value = ranges[HUMIDITY_TO_LOCATION_IDX].get_original_input_value(curr_loc);

        let temp_value = ranges[TEMP_TO_HUMIDITY_IDX].get_original_input_value(humidity_value);

        let light_value = ranges[LIGHT_TO_TEMP_IDX].get_original_input_value(temp_value);

        let water_value = ranges[WATER_TO_LIGHT_IDX].get_original_input_value(light_value);
        let fertilizer_value =
            ranges[FERTILIZER_TO_WATER_IDX].get_original_input_value(water_value);

        let soil_value = ranges[SOIL_TO_FERTILIZER_IDX].get_original_input_value(fertilizer_value);
        let seed_value = ranges[SEED_TO_SOIL_IDX].get_original_input_value(soil_value);
        let valid_seed_value = starting_seed_range.get_original_input_value_if_exists(seed_value);
        // println!("CURR_LOC: {:?}, SEED: {:?}", curr_loc, seed_value);
        if valid_seed_value == -1 {
            curr_loc += 1;
            continue;
        }
        return curr_loc;
    }

    // println!("STARTING SEEDS: {:?}", starting_seed_vec);
    // let r = ranges.get(0).unwrap().is_valid_dest(49);
    // println!("{:?}", r);
    // println!("STEP 1");
    // let soil_values: Vec<i64> = starting_seed_vec
    //     .into_iter()
    //     .map(|seed| {
    //         ranges
    //             .get(SEED_TO_SOIL_IDX)
    //             .unwrap()
    //             .get_final_output_value(seed)
    //     })
    //     .collect();
    // println!("STEP 2");
    // let fertilizer_values: Vec<i64> = soil_values
    //     .into_iter()
    //     .map(|soil| {
    //         ranges
    //             .get(SOIL_TO_FERTILIZER_IDX)
    //             .unwrap()
    //             .get_final_output_value(soil)
    //     })
    //     .collect();
    // println!("STEP 3");
    // let water_values: Vec<i64> = fertilizer_values
    //     .into_iter()
    //     .map(|fertilizer| {
    //         ranges
    //             .get(FERTILIZER_TO_WATER_IDX)
    //             .unwrap()
    //             .get_final_output_value(fertilizer)
    //     })
    //     .collect();
    // println!("STEP 4");
    // let light_values: Vec<i64> = water_values
    //     .into_iter()
    //     .map(|light| {
    //         ranges
    //             .get(WATER_TO_LIGHT_IDX)
    //             .unwrap()
    //             .get_final_output_value(light)
    //     })
    //     .collect();
    // println!("STEP 5");
    // let temp_values: Vec<i64> = light_values
    //     .into_iter()
    //     .map(|light| {
    //         ranges
    //             .get(LIGHT_TO_TEMP_IDX)
    //             .unwrap()
    //             .get_final_output_value(light)
    //     })
    //     .collect();
    // println!("STEP 6");
    // let humidity_values: Vec<i64> = temp_values
    //     .into_iter()
    //     .map(|temp| {
    //         ranges
    //             .get(TEMP_TO_HUMIDITY_IDX)
    //             .unwrap()
    //             .get_final_output_value(temp)
    //     })
    //     .collect();
    // println!("STEP 7");
    // let location_values: Vec<i64> = humidity_values
    //     .into_iter()
    //     .map(|humidity| {
    //         ranges
    //             .get(HUMIDITY_TO_LOCATION_IDX)
    //             .unwrap()
    //             .get_final_output_value(humidity)
    //     })
    //     .collect();
    // println!("STEP 8");
    // location_values.into_iter().min().unwrap()
}

pub fn day5() -> Answer {
    let lines = read_file("./src/day5/input.txt");
    let result: i64 = handle_part_2(lines);
    // let result = 0;
    // let input_range = InputRange {
    //     source_start: 50,
    //     dest_start: 52,
    //     range_length: 48,
    //     current_iter_value: 0,
    // };
    // for x in input_range.into_iter() {
    //     println!("{:?}", x);
    // }
    // println!("{:?}", input_range);
    // for value in 0..100 {
    //     println!("{:?} -> {:?}", value, input_range.get_output_value(value));
    // }

    // for (index, line) in lines.enumerate() {
    // }

    Answer::Int(result)
}

fn read_file(filename: &str) -> impl Iterator<Item = Result<String, io::Error>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
