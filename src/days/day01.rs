use crate::days::Day;
use crate::util::number;

pub const DAY1: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let result: Result<Vec<u128>, String> = input.lines().map(|l| number::parse_u128(l)).collect();
    let depths = match result {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(nums) => { nums }
    };

    // Puzzle 1
    // The first order of business is to figure out how quickly the depth increases,
    // just so you know what you're dealing with - you never know if the keys will
    // get carried into deeper water by an ocean current or a fish or something.
    //
    // To do this, count the number of times a depth measurement increases from the
    // previous measurement. (There is no measurement before the first measurement.)

    let mut increases = 0;
    let mut last_depth = &depths[0];
    for depth in &depths[1..] {
        if depth > last_depth {
            increases += 1;
        }
        last_depth = depth;
    }

    println!("Puzzle 1 answer: {}", increases);
}

fn puzzle2(input: &String) {
    let result: Result<Vec<u128>, String> = input.lines().map(|l| number::parse_u128(l)).collect();
    let depths = match result {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(nums) => { nums }
    };

    // Same as above, but using a sliding window summing three values
    // A = 0,1,2
    // B = 1,2,3
    // ...
    // Z = N-3, N-2, N-1 (// for N values)
    let mut last_window: u128 = depths[0..3].iter().sum();
    let mut increases = 0;
    for i in 1..depths.len() - 2 {
        let window = depths[i..i + 3].iter().sum();
        println!("Last window: {}, new window: {}", last_window, window);
        if window > last_window { increases += 1; }
        last_window = window;
    }

    println!("Puzzle 2 answer: {}", increases);
}