use std::{collections::HashMap, fs::read_to_string};

use anyhow::Result;
use log::debug;
use nom::{
    bytes::complete::{tag, take_while},
    character::{complete::digit1, is_alphabetic},
    combinator::map_res,
    multi::separated_list0,
    sequence::{separated_pair, tuple},
    IResult,
};
use shared::init_app;

const CONSTRAINTS: &[(u32, &str); 3] = &[(12, "red"), (13, "green"), (14, "blue")];

fn main() -> Result<()> {
    init_app(1);

    let input = read_to_string("/home/nuke/temp/advent_of_code/02.txt")?;

    part_one(&input);
    part_two(&input);

    Ok(())
}

fn part_one(input: &str) {
    let mut valid_game_ids: Vec<u32> = Vec::new();
    'lines: for line in input.lines() {
        debug!("Line: {line}");
        let (line, (_, game_id, _)) = parse_game(line).unwrap();
        let (_, rounds) = separated_list0(tag("; "), parse_colors)(line).unwrap();

        for seen_colors in rounds {
            for (max_amount, color) in CONSTRAINTS {
                // Check if the color exists in the first place
                if let Some(actual_amount) = seen_colors.get(&color.to_string()) {
                    if actual_amount > max_amount {
                        continue 'lines;
                    }
                }
            }
        }

        valid_game_ids.push(game_id);
    }

    let id_sum: u32 = valid_game_ids.iter().sum();
    println!("Sum of all valid game ids: {id_sum}");
}

fn part_two(input: &str) {
    let mut min_color_sets = Vec::new();
    for line in input.lines() {
        debug!("Line: {line}");
        let (line, (_, _, _)) = parse_game(line).unwrap();
        let (_, rounds) = separated_list0(tag("; "), parse_colors)(line).unwrap();

        let mut min_color_set = HashMap::new();
        for seen_colors in rounds {
            for (name, amount) in seen_colors {
                min_color_set
                    .entry(name)
                    .and_modify(|value| {
                        if amount > *value {
                            *value = amount
                        }
                    })
                    .or_insert(amount);
            }
        }
        debug!("Min color set: {min_color_set:#?}");
        min_color_sets.push(min_color_set);
    }

    let sum: u32 = min_color_sets
        .into_iter()
        .map(|set| set.into_values().product::<u32>())
        .sum();
    println!("Sum of lowest cube numers per game: {sum}");
}

fn parse_game(input: &str) -> IResult<&str, (&str, u32, &str)> {
    tuple((tag("Game "), map_res(digit1, str::parse), tag(": ")))(input)
}

fn parse_colors(input: &str) -> IResult<&str, HashMap<String, u32>> {
    let (input, colors) = separated_list0(tag(", "), parse_color)(input)?;

    Ok((input, HashMap::from_iter(colors)))
}

fn parse_color(input: &str) -> IResult<&str, (String, u32)> {
    let (input, (amount, name)) =
        separated_pair(digit1, tag(" "), take_while(|c| is_alphabetic(c as u8)))(input)?;
    Ok((
        input,
        (
            name.to_string(),
            amount.parse().expect("Found invalid number"),
        ),
    ))
}
