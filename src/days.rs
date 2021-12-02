pub struct Day {
    pub puzzle1: fn(input: &String),
    pub puzzle2: fn(input: &String)
}

mod day01;
use day01::DAY1;

pub fn get_day(day: i32) -> Result<Day, String> {
    match day {
        1 => Ok(DAY1),
        _ => Err(format!("No implementation yet for day {}", day))
    }
}