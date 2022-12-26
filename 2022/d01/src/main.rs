fn main() {
    let input = include_str!("input.txt");

    let mut max_calories = 0;
    let mut current_calories = 0;

    for line in input.lines() {
        // An empty line marks the end of the current elv's inventory.
        // Check if they have more calories than the current maximum.
        if line.trim() == "" {
            // If so, set the next elv
            if current_calories > max_calories {
                max_calories = current_calories;
            }
            // Reset the calory counter for the next elv
            current_calories = 0;
            continue;
        }

        let parse_result = line.trim().parse();
        let calories: i32 = match parse_result {
            Ok(calories) => calories,
            Err(error) => panic!("Got invalid line {line} with error {error:?}"),
        };

        current_calories += calories;
    }

    // Check one last time in case the last line is no newline.
    if current_calories > max_calories {
        max_calories = current_calories;
    }

    println!("Max calories: {max_calories}");
}
