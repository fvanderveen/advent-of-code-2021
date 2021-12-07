use std::cmp::{min};
use crate::days::Day;
use crate::util::number;

pub const DAY6: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let fish = match parse_input(input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };

    let result = get_fish_after_days(&fish, 80);
    println!("Puzzle 1 answer: {}", result);
}

fn puzzle2(input: &String) {
    let fish = match parse_input(input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };

    let result = get_fish_after_days(&fish, 256);
    println!("Puzzle 2 answer: {}", result);
}

fn parse_input(input: &str) -> Result<Vec<u128>, String> {
    input.split(",").map(|i| number::parse_u128(i)).collect()
}

fn get_fish_after_days(initial_state: &Vec<u128>, num_days: u128) -> u128 {
    // Running the simulation works for small amount of days, but these fish go fast.
    // This function should be a slightly smarter way to reason about them.
    // How would one optimize this?
    // Technically, it's easy to calculate how many fish a given fish will spawn
    // 0<=N<=8 => floor((num_days - N) / 7) + 1
    // However, each of those spawned fish will also spawn new fish (9 days after spawn)
    // D 0 1 2 3 4 5 6 7 8 9 A B C D ...
    // A 3 2 1 0 6 5 4 3 2 1 0 6 5 4 ...
    //         B 8 7 6 5 4 3 2 1 0 6 ...
    //                       C 8 7 6 ...
    //                           D 8 ...

    // In the end, all fish will be in one of 7 buckets; and all of those will spawn similar fish.

    let mut buckets: [u128; 9] = [0; 9];
    for state in initial_state {
        buckets[state.clone() as usize] += 1;
    }

    let mut days_left = num_days;
    while days_left > 0 {
        buckets = simulate_bucket(buckets, min(7, days_left));
        days_left = if days_left > 7 { days_left - 7 } else { 0 };
    }

    buckets.iter().sum()
}

fn simulate_bucket(buckets: [u128; 9], num_days: u128) -> [u128; 9] {
    let mut new_buckets: [u128; 9] = [0; 9];
    let sim_days = min(num_days as usize, 7);
    let offset = 8 - sim_days + 1;

    // Create initial state:
    // Copy what will be handled
    for i in 0..sim_days {
        new_buckets[i] = buckets[i];
    }
    // Move what will not be handled
    for i in sim_days..9 {
        new_buckets[i - sim_days] += buckets[i];
    }

    // Spawn new fishies:
    for i in 0..sim_days {
        let target = i + offset;
        new_buckets[target] += buckets[i]; // This summation is also not correct for num_days != 7?
    }

    new_buckets
}

#[cfg(test)]
mod tests {
    use crate::days::day06::{get_fish_after_days, parse_input};

    const EXAMPLE_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(EXAMPLE_INPUT), Ok(vec![3, 4, 3, 1, 2]));
    }

    #[test]
    fn test_get_fish_after_days() {
        let input = parse_input(EXAMPLE_INPUT).unwrap();

        assert_eq!(get_fish_after_days(&input, 18), 26);
        assert_eq!(get_fish_after_days(&input, 80), 5934);
        assert_eq!(get_fish_after_days(&input, 256), 26984457539);
    }

    #[test]
    fn debug_test_iteration_days() {
        for day in (3..80).step_by(7) {
            println!("{}", day)
        }
    }
}