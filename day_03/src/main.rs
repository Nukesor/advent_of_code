use std::{fs::read_to_string, cmp::max};

use anyhow::Result;
use log::debug;
use shared::init_app;

type Grid = [[char; 140]; 140];

fn main() -> Result<()> {
    init_app(1);

    let input = read_to_string("/home/nuke/temp/advent_of_code/03.txt")?;
    let grid = init_grid(input);

    part_one(grid);

    Ok(())
}

/// Take the file content and load it into a 140x140 array.
fn init_grid(input: String) -> Grid {
    let mut grid: Grid = [['.'; 140]; 140];

    for (index, line) in input.lines().enumerate() {
        grid[index] = line.chars().collect::<Vec<char>>().try_into().unwrap();
    }

    for line in grid {
        for c in line {
            debug!("{c}");
        }
        debug!("");
    }

    grid
}

fn part_one(grid: Grid) {
    let mut valid_numbers: Vec<u32> = Vec::new();
    for (row_index, line) in grid.iter().enumerate() {
        let mut number_complete = false;
        let mut number_is_valid = false;
        let mut current_number = String::new();
        let mut last_char = line[0];

        // Iterate through all chars of this line.
        for (c_index, c) in line.iter().enumerate() {
            // If we hit the first digit of a number, check if the previous char was a symbol.
            // If that's the case, immediately see the number as valid.
            if c.is_ascii_digit() {
                if current_number.is_empty() && last_char != '.' {
                    number_is_valid = true;
                    current_number.push(*c)
                }
            } else {
                // Check if we were in a number and should terminate the number sequence.
                if !current_number.is_empty() {
                    // Also immediately set the number as accepted, if the field after it is a symbol.
                    if *c != '.' {
                        number_is_valid = true
                    }

                    number_complete = true;
                }
            }

            // We hit a new char or the end of the line.
            // Let's see if the number is valid.
            if number_complete || c_index == 139 {
                // Check the surrounding
                if !number_is_valid {
                    // Determine the start of the number word.
                    let number_start = if c_index == 139 {
                        139 - current_number.len()
                    } else {
                        c_index - current_number.len() - 1
                    };

                    number_is_valid = check_surrounding_elements(
                        &grid,
                        row_index,
                        number_start,
                        current_number.len(),
                    );
                }

                if number_is_valid {
                    valid_numbers.push(current_number.parse().unwrap());
                    current_number = String::new();
                    number_is_valid = false;

                    last_char = *c;
                    continue;
                }
            }
        }
    }
}

/// Check the surrounding elements of a given number.
/// This needs to handle several edge cases, if the number is on any side of the grid.
fn check_surrounding_elements(
    grid: &Grid,
    row_index: usize,
    number_start: usize,
    number_len: usize,
) -> bool {
    // Check chars on top of the number (if not in the first line.
    let surrounding_start = max(number_start, number_start - 1);
    let surrounding_stop = min(139, number_start+number_len + 1);
    if row_index != 0 {
        for index in surrounding_start..surrounding_stop {
            let
        }
    }

    false
}

fn check_
