use std::fs::read_to_string;

use anyhow::Result;
use log::debug;
use shared::init_app;

const WORD_DIGITS: [(u32, &str); 9] = [
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

fn main() -> Result<()> {
    init_app(1);

    let input = read_to_string("/home/nuke/temp/advent_of_code/01.txt")?;
    part_one(&input)?;

    // Scan the whole word for all digits.
    // We then take the the first and the last digit.
    //
    // This isn't the most efficient, but it should be fast enough.
    // The alternative would be lots of hardcoding, which I'm not keen on.
    let mut sum = 0;
    for line in input.lines() {
        debug!("Line: {line}");
        let line: Vec<char> = line.chars().collect();

        let mut digits: Vec<u32> = Vec::new();
        // Iterate over all chars
        let mut current_index = 0;
        while current_index < line.len() {
            // Check if it's a straight up digit.
            // If so, take it and check the next digit
            if line[current_index].is_ascii_digit() {
                digits.push(line[current_index].to_digit(10).unwrap());
                current_index += 1;
                continue;
            }

            // Check if there's a word digit from the current position forward.
            // If we find one, we take the first char after the word as the new index and push the
            // digit.
            if let Some(digit) = scan_for_digit_word(&line, current_index) {
                digits.push(digit);
                current_index += 1;
                continue;
            }

            // We didn't find anything, check the next char.
            current_index += 1;
        }

        let value: u32 =
            format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse()?;
        debug!("Value: {value}");
        sum += value
    }

    println!("Sum: {sum}");

    Ok(())
}

/// Scan if there's any word in the string that starts from the current position.
///
/// This isn't the most efficient, but it should be fast enough.
/// The alternative would be lots of hardcoding, which I'm not keen on.
///
/// Returns `found_integer`
fn scan_for_digit_word(line: &[char], index: usize) -> Option<u32> {
    for (number, word) in WORD_DIGITS {
        let word: Vec<char> = word.chars().collect();
        let word_length = word.len();

        // Get subslice with `word_length` from the current position.
        // Is `None` if there're no more chars.
        let subslice = line.get(index..(index + word_length));
        if subslice == Some(&word) {
            return Some(number);
        }
    }

    None
}

fn part_one(input: &str) -> Result<()> {
    let mut sum = 0;
    for line in input.lines() {
        let first = line.chars().find(|c| c.is_ascii_digit()).unwrap();
        let last = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();

        debug!("Line: {line}");
        let value: u32 = format!("{first}{last}").parse()?;
        debug!("Value: {value}");
        sum += value
    }

    println!("Part one sum: {sum}");

    Ok(())
}
