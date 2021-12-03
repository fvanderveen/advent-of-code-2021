use crate::days::Day;

pub const DAY3: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    // Input are binary numbers
    // Calculate gamma and epsilon
    // gamma = reduce bits by taking the most frequent, epsilon takes less frequent (both are inverse of each other)
    // answer is multiplying the resulting numbers in decimal form
    match calculate_power_consumption(input) {
        Ok(consumption) => {
            let result = consumption.gamma * consumption.epsilon;
            println!("Puzzle 1 result: {}", result);
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

fn puzzle2(input: &String) {
    let oxygen_rating = match calculate_oxygen_rating(input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };
    let co2_rating = match calculate_co2_rating(input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };

    let result = oxygen_rating * co2_rating;
    println!("Puzzle 2 result: {}", result);
}

#[derive(Eq, PartialEq, Debug)]
struct PowerConsumption {
    gamma: u128,
    epsilon: u128,
}

fn calculate_power_consumption(data: &str) -> Result<PowerConsumption, String> {
    let inputs: Vec<&str> = data.lines().collect();
    let word_size = match inputs.get(0) {
        Some(w) => w.len(),
        _ => return Err("No input given".to_string())
    };
    let mut counters: Vec<u128> = vec![0; word_size];

    for input in &inputs {
        let chars: Vec<char> = input.chars().collect();
        for i in 0..word_size {
            counters[i] += match chars.get(i) {
                Some('1') => 1,
                Some('0') => 0,
                other => return Err(format!("Invalid binary character in '{}': {:?}", input, other))
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..word_size {
        gamma <<= 1;
        epsilon <<= 1;
        if counters[i] * 2 > inputs.len() as u128 {
            // More 1's than 0's
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    Ok(PowerConsumption { gamma, epsilon })
}

fn get_most_common_bit(data: &Vec<&str>, bit: usize) -> Result<char, String> {
    let result: Result<Vec<usize>, String> = data.into_iter()
        .map(|i| match i.chars().nth(bit) {
            Some('1') => Ok(1),
            Some('0') => Ok(0),
            _ => Err(format!("Invalid input data in {} at {}", i, bit))
        })
        .collect();

    let counter: usize = match result {
        Ok(v) => v.into_iter().sum(),
        Err(e) => return Err(e)
    };

    if counter * 2 >= data.len() { Ok('1') } else { Ok('0') }
}

fn binary_to_number(binary: &str) -> Result<u128, String> {
    let mut result = 0;
    for char in binary.chars() {
        result <<= 1;
        match char {
            '1' => result += 1,
            '0' => {},
            _ => return Err(format!("Invalid binary character '{}' in '{}'", char, binary))
        }
    }

    Ok(result)
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum RatingType {
    OXYGEN,
    CO2
}

fn get_rating(data: &str, rating: RatingType) -> Result<u128, String> {
    let mut inputs: Vec<&str> = data.lines().collect();

    let word_length = match inputs.get(0) {
        Some(w) => w.len(),
        None => return Err("Expected inputs...".to_string())
    };

    for i in 0..word_length {
        let target_bit = match get_most_common_bit(&inputs, i) {
            Ok(c) => c,
            Err(e) => return Err(e)
        };
        inputs = inputs.into_iter().filter(|input| match input.chars().nth(i) {
            Some(c) => (c == target_bit) == (rating == RatingType::OXYGEN),
            None => false
        }).collect();
        if inputs.len() == 1 { break; }
    }

    match inputs.len() {
        1 => binary_to_number(inputs[0]),
        len => Err(format!("Did not reduce inputs to a single value, kept {}", len))
    }
}

fn calculate_oxygen_rating(data: &str) -> Result<u128, String> {
    // Check per bit, keep only those with the most frequent occurrence (equal numbers = use 1)
    // Once one is left, that's the value.
    get_rating(data, RatingType::OXYGEN)
}

fn calculate_co2_rating(data: &str) -> Result<u128, String> {
    // Check per bit, keep only those with the least frequent occurrence (equal numbers = use 1)
    // Once one is left, that's the value.
    get_rating(data, RatingType::CO2)
}

#[cfg(test)]
mod tests {
    use crate::days::day03::{binary_to_number, calculate_co2_rating, calculate_oxygen_rating, calculate_power_consumption, get_most_common_bit};

    const EXAMPLE_INPUT: &str = "00100\n\
                                 11110\n\
                                 10110\n\
                                 10111\n\
                                 10101\n\
                                 01111\n\
                                 00111\n\
                                 11100\n\
                                 10000\n\
                                 11001\n\
                                 00010\n\
                                 01010";

    #[test]
    fn power_consumption_example() {
        let result = calculate_power_consumption(EXAMPLE_INPUT);
        println!("{:?}", result);
        assert!(result.is_ok(), "expected result to be ok");
        let consumption = result.unwrap();
        assert_eq!(consumption.gamma, 22);
        assert_eq!(consumption.epsilon, 9);
    }

    #[test]
    fn get_common_bit() {
        let example_data = EXAMPLE_INPUT.lines().collect();
        assert_eq!(get_most_common_bit(&example_data, 0), Ok('1'));
        assert_eq!(get_most_common_bit(&example_data, 1), Ok('0'));
        assert_eq!(get_most_common_bit(&example_data, 2), Ok('1'));
        assert_eq!(get_most_common_bit(&example_data, 3), Ok('1'));
    }

    #[test]
    fn oxygen_rating_example() {
        let result = calculate_oxygen_rating(EXAMPLE_INPUT);
        println!("{:?}", result);
        assert!(result.is_ok());
        let rating = result.unwrap();
        assert_eq!(rating, 23);
    }

    #[test]
    fn co2_rating_example() {
        let result = calculate_co2_rating(EXAMPLE_INPUT);
        println!("{:?}", result);
        assert!(result.is_ok());
        let rating = result.unwrap();
        assert_eq!(rating, 10);
    }

    #[test]
    fn binary_to_number_tests() {
        assert_eq!(binary_to_number("10110"), Ok(22));
        assert_eq!(binary_to_number("00001"), Ok(1));
        assert_eq!(binary_to_number("00000"), Ok(0));
    }
}