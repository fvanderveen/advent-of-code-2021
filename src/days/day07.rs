use crate::days::Day;
use crate::util::number;

pub const DAY7: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let state = match parse_input(input) {
        Err(e) => panic!("{}", e),
        Ok(v) => v
    };

    let result = get_cheapest_position(&state, &DistanceMode::Puzzle1);

    println!("Puzzle 1 answer: {}", result.fuel);
}

fn puzzle2(input: &String) {
    let state = match parse_input(input) {
        Err(e) => panic!("{}", e),
        Ok(v) => v
    };

    let result = get_cheapest_position(&state, &DistanceMode::Puzzle2);

    println!("Puzzle 2 answer: {} ({})", result.fuel, result.value);
}

fn parse_input(input: &str) -> Result<Vec<i128>, String> {
    input.split(",").map(|p| number::parse_i128(p)).collect()
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Position {
    value: i128,
    fuel: i128
}

fn get_cheapest_position(initial_state: &Vec<i128>, mode: &DistanceMode) -> Position {
    // Find the position that takes the least amount of total steps to make all initial_state values equal
    // Brute force is most likely not going to work for the real input (or otherwise puzzle 2)
    // Starting at the median makes sense, as it'll account for outliers.
    // From there it's moving to the side that _reduces_ the total difference, until it no longer reduces.
    // And hope for the best? :D

    let mut sorted_state = initial_state.clone().to_vec();
    sorted_state.sort();

    let initial_guess = match mode {
        DistanceMode::Puzzle1 => sorted_state[sorted_state.len() / 2],
        DistanceMode::Puzzle2 => {
            // For puzzle 2 it makes more sense to start at the average position
            let total = initial_state.iter().map(|v| v.clone() as f64).sum::<f64>();
            let average = total / (initial_state.len() as f64);
            println!("Using average: {}, initial_guess: {}", average, average.round() as i128);
            average.round() as i128
        }
    };
    let initial_distance = get_total_fuel(initial_guess, initial_state, mode);

    let lower = try_lower(initial_guess, initial_distance, initial_state, mode);
    let higher = try_higher(initial_guess, initial_distance, initial_state, mode);

    if lower.fuel < higher.fuel { lower } else { higher }
}

fn try_lower(initial_guess: i128, current_minimum: i128, initial_state: &Vec<i128>, mode: &DistanceMode) -> Position {
    let lower_guess = initial_guess - 1;
    let lower_distance = get_total_fuel(lower_guess, initial_state, mode);
    if lower_distance > current_minimum {
        Position { value: initial_guess, fuel: current_minimum }
    } else {
        try_lower(lower_guess, lower_distance, initial_state, mode)
    }
}

fn try_higher(initial_guess: i128, current_minimum: i128, initial_state: &Vec<i128>, mode: &DistanceMode) -> Position {
    let higher_guess = initial_guess + 1;
    let higher_distance = get_total_fuel(higher_guess, initial_state, mode);
    if higher_distance > current_minimum {
        Position { value: initial_guess, fuel: current_minimum }
    } else {
        try_higher(higher_guess, higher_distance, initial_state, mode)
    }
}

enum DistanceMode {
    Puzzle1,
    Puzzle2,
}

fn get_total_fuel(position: i128, initial_states: &Vec<i128>, mode: &DistanceMode) -> i128 {
    match mode {
        DistanceMode::Puzzle1 => initial_states.iter().map(|v| (v - position).abs()).sum(),
        DistanceMode::Puzzle2 => initial_states.iter().map(|v| (0..(v - position).abs() + 1).sum::<i128>()).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day07::{get_cheapest_position, get_total_fuel, Position};
    use crate::days::day07::DistanceMode::{Puzzle1, Puzzle2};

    const EXAMPLE_INPUT: [i128; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_get_total_fuel() {
        assert_eq!(get_total_fuel(1, &EXAMPLE_INPUT.to_vec(), &Puzzle1), 41);
        assert_eq!(get_total_fuel(2, &EXAMPLE_INPUT.to_vec(), &Puzzle1), 37);
        assert_eq!(get_total_fuel(3, &EXAMPLE_INPUT.to_vec(), &Puzzle1), 39);
        assert_eq!(get_total_fuel(10, &EXAMPLE_INPUT.to_vec(), &Puzzle1), 71);

        assert_eq!(get_total_fuel(2, &EXAMPLE_INPUT.to_vec(), &Puzzle2), 206);
        assert_eq!(get_total_fuel(5, &EXAMPLE_INPUT.to_vec(), &Puzzle2), 168);
    }

    #[test]
    fn test_get_cheapest_position() {
        assert_eq!(get_cheapest_position(&EXAMPLE_INPUT.to_vec(), &Puzzle1), Position { value: 2, fuel: 37 });
        assert_eq!(get_cheapest_position(&EXAMPLE_INPUT.to_vec(), &Puzzle2), Position { value: 5, fuel: 168 });
    }
}