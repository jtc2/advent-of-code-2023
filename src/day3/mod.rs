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

#[derive(Copy, Clone, Debug, Default)]
struct GridSpot {
    numeric_value: i32,
    value: char,
    is_symbol: bool,
    is_dot: bool,
    visited: bool,
    // If this is a number, how many grid spots to the left/right are also part of the number
    left_count: i32,
    right_count: i32,
}

fn get_index_of_num_end(line: String) -> i32 {
    let mut index = 0;
    for chr in line.chars() {
        if chr.is_digit(10) {
            index += 1
        } else {
            break;
        }
    }
    return index;
}

fn parse_grid_line(line: String) -> Vec<GridSpot> {
    let mut grid_spots: Vec<GridSpot> = vec![];

    let mut skip_count = 0;
    for (index, chr) in line.chars().enumerate() {
        // Handle spots that are already in the grid
        if skip_count >= 1 {
            skip_count -= 1;
            continue;
        }

        if chr == '.' {
            // If here, empty spot
            grid_spots.push(GridSpot {
                numeric_value: 0,
                value: chr,
                is_symbol: false,
                is_dot: true,
                visited: false,
                left_count: 0,
                right_count: 0,
            })
        } else if chr.is_digit(10) {
            // If here, this is the first digit of the number -> need to look ahead
            let next_dot_index: i32 =
                get_index_of_num_end(line[index..].to_string()) + index as i32;
            let value = line[index..next_dot_index as usize].parse::<i32>().unwrap();
            // Skip the extra digits and push the grid spot that many times, changing only the left/right count
            let num_digits: i32 = next_dot_index - index as i32;
            skip_count += num_digits - 1;

            let mut curr_grid_spot = GridSpot {
                numeric_value: value,
                value: chr,
                is_symbol: false,
                is_dot: false,
                visited: false,
                left_count: 0,
                right_count: num_digits - 1,
            };
            grid_spots.push(curr_grid_spot);
            for _ in 1..num_digits {
                let mut next_grid_spot = curr_grid_spot.clone();
                next_grid_spot.left_count += 1;
                next_grid_spot.right_count -= 1;
                grid_spots.push(next_grid_spot);
                curr_grid_spot = next_grid_spot;
            }
        } else {
            // If here, is a symbol
            grid_spots.push(GridSpot {
                numeric_value: 0,
                value: chr,
                is_symbol: true,
                is_dot: false,
                visited: false,
                left_count: 0,
                right_count: 0,
            })
        }
    }
    grid_spots
}

// Fuck this function
fn calculate_grid_value(grid: &mut Vec<Vec<GridSpot>>) -> i32 {
    let mut total_part_value: i32 = 0;

    let cloned_grid = grid.clone();
    let num_rows: i32 = grid.len() as i32;
    let num_cols: i32 = grid[0].len() as i32;
    for (row_idx, row) in cloned_grid.into_iter().enumerate() {
        for (col_idx, grid_spot) in row.into_iter().enumerate() {
            // Don't mark numbers visited until they are added, but mark everything else visited
            if grid_spot.numeric_value == 0 {
                // symbol or dot
                grid[row_idx][col_idx].visited = true;
            }

            if grid_spot.is_dot || grid_spot.numeric_value != 0 {
                // Ignore dots and numbers
                continue;
            }

            if grid_spot.is_symbol {
                // At this point, check all 8 directions to see if there's an unvisited number and if so,
                // add to the total_part_value and mark that number visited
                for row_change in -1..2 {
                    for col_change in -1..2 {
                        println!("{:?} {:?}", row_change, col_change);
                        // Same spot - ignore
                        if row_change == 0 && col_change == 0 {
                            continue;
                        }

                        // Out of bounds checks
                        let next_row = row_idx as i32 + row_change;
                        let next_col = col_idx as i32 + col_change;
                        if next_row < 0
                            || next_row >= num_rows
                            || next_col < 0
                            || next_col >= num_cols
                        {
                            println!("Continuing???");
                            continue;
                        }

                        let relative_spot = &mut grid[next_row as usize][next_col as usize];
                        println!("MY RELATIVE: {:?}", relative_spot);
                        if relative_spot.visited || relative_spot.is_dot || relative_spot.is_symbol
                        {
                            println!("Visited relative????, {:?}", relative_spot);
                            continue;
                        }

                        // We are in an unvisited number
                        // First add the part
                        println!("HERE I AM!");
                        total_part_value += relative_spot.numeric_value;

                        // Mark the next spot as visited
                        grid[next_row as usize][next_col as usize].visited = true;

                        // Mark the left digits as visited
                        for col_offset in 0..grid[next_row as usize][next_col as usize]
                            .left_count
                            .clone()
                        {
                            println!("LEFT!!!!!");
                            grid[next_row as usize][(next_col - col_offset) as usize].visited =
                                true;
                        }

                        // Mark the right digits as visited
                        for col_offset in 0..grid[next_row as usize][next_col as usize]
                            .right_count
                            .clone()
                        {
                            println!("RIGHT!!!!!!!");
                            grid[next_row as usize][(next_col + col_offset) as usize].visited =
                                true;
                        }
                    }
                }
            }
        }
    }
    total_part_value
}

fn calculate_grid_value_v2(grid: &mut Vec<Vec<GridSpot>>) -> i32 {
    // Note v2 is for part 1
    let mut total_part_value: i32 = 0;

    let num_rows = grid.len();
    let num_cols = grid[0].len();
    for row_idx in 0..num_rows {
        for col_idx in 0..num_cols {
            // Now the grid is fucking editable
            let curr_spot = &mut grid[row_idx][col_idx];

            // Don't mark numbers visited until they are added, but mark everything else visited
            if curr_spot.numeric_value == 0 {
                // symbol or dot
                curr_spot.visited = true;
            }

            if curr_spot.is_dot || curr_spot.numeric_value != 0 {
                // Ignore dots and numbers
                continue;
            }

            if curr_spot.is_symbol {
                // At this point, check all 8 directions to see if there's an unvisited number and if so,
                // add to the total_part_value and mark that number visited
                for row_change in -1..2 {
                    for col_change in -1..2 {
                        // Same spot - ignore
                        if row_change == 0 && col_change == 0 {
                            continue;
                        }

                        // Out of bounds checks
                        let next_row = row_idx as i32 + row_change;
                        let next_col = col_idx as i32 + col_change;
                        if next_row < 0
                            || next_row >= num_rows as i32
                            || next_col < 0
                            || next_col >= num_cols as i32
                        {
                            continue;
                        }

                        let relative_spot = &mut grid[next_row as usize][next_col as usize];
                        if relative_spot.visited || relative_spot.is_dot || relative_spot.is_symbol
                        {
                            continue;
                        }

                        // We are in an unvisited number
                        // First add the part
                        total_part_value += relative_spot.numeric_value;

                        // Mark the next spot as visited
                        relative_spot.visited = true;
                        let relative_left_count = relative_spot.left_count;
                        let relative_right_count = relative_spot.right_count;

                        // Mark the left digits as visited
                        for col_offset in 0..relative_left_count {
                            grid[next_row as usize][(next_col - col_offset + 1) as usize].visited =
                                true;
                        }

                        // Mark the right digits as visited
                        for col_offset in 0..relative_right_count {
                            grid[next_row as usize][(next_col + col_offset + 1) as usize].visited =
                                true;
                        }
                    }
                }
            }
        }
    }
    total_part_value
}

fn calculate_grid_value_v21(grid: &mut Vec<Vec<GridSpot>>) -> i32 {
    // Note v21 is actually v2.1 but rust won't allow dots in function names, dumb language
    // This is for part 2
    let mut total_gear_value: i32 = 0;

    let num_rows = grid.len();
    let num_cols = grid[0].len();
    for row_idx in 0..num_rows {
        for col_idx in 0..num_cols {
            // Now the grid is fucking editable
            let curr_spot = &mut grid[row_idx][col_idx];

            // Don't mark numbers visited until they are added, but mark everything else visited
            if curr_spot.numeric_value == 0 {
                // symbol or dot
                curr_spot.visited = true;
            }

            if curr_spot.is_dot || curr_spot.numeric_value != 0 {
                // Ignore dots and numbers
                continue;
            }

            if curr_spot.is_symbol && curr_spot.value == '*' {
                // At this point, check all 8 directions to see if there's an unvisited number and if so,
                // add to the total_part_value and mark that number visited

                let mut nearby_nums: Vec<i32> = vec![];
                for row_change in -1..2 {
                    for col_change in -1..2 {
                        // Same spot - ignore
                        if row_change == 0 && col_change == 0 {
                            continue;
                        }

                        // Out of bounds checks
                        let next_row = row_idx as i32 + row_change;
                        let next_col = col_idx as i32 + col_change;
                        if next_row < 0
                            || next_row >= num_rows as i32
                            || next_col < 0
                            || next_col >= num_cols as i32
                        {
                            continue;
                        }

                        let relative_spot = &mut grid[next_row as usize][next_col as usize];
                        if relative_spot.visited || relative_spot.is_dot || relative_spot.is_symbol
                        {
                            continue;
                        }

                        // We are in an unvisited number
                        // First add the part
                        nearby_nums.push(relative_spot.numeric_value);

                        // Mark the next spot as visited
                        relative_spot.visited = true;
                        let relative_left_count = relative_spot.left_count;
                        let relative_right_count = relative_spot.right_count;

                        // Mark the left digits as visited
                        for col_offset in 0..relative_left_count {
                            grid[next_row as usize][(next_col - col_offset + 1) as usize].visited =
                                true;
                        }

                        // Mark the right digits as visited
                        for col_offset in 0..relative_right_count {
                            grid[next_row as usize][(next_col + col_offset + 1) as usize].visited =
                                true;
                        }
                    }
                }
                if nearby_nums.len() == 2 {
                    total_gear_value += nearby_nums[0] * nearby_nums[1];
                }
            }
        }
    }
    total_gear_value
}

pub fn day3() -> Answer {
    let lines = read_file("./src/day3/input.txt");
    let mut full_grid: Vec<Vec<GridSpot>> = vec![];
    for line in lines {
        full_grid.push(parse_grid_line(line.unwrap()));
    }
    let result = calculate_grid_value_v21(&mut full_grid);
    Answer::Int(result)
}

fn read_file(filename: &str) -> impl Iterator<Item = Result<String, io::Error>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().into_iter()
}
