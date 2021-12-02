mod days;
mod util;

use std::env::args;
use days::{get_day, Day};
use util::input::{read_input};
use util::number::{parse_i32};

fn print_usage()
{
    eprintln!("
Usage: cargo run <command> [<command_arg>, ...]

Commands:
    day <day number> - run the puzzles for the given day.
");
}

fn main() {
    let a: Vec<String> = args().collect();

    if a.len() < 3 {
        print_usage();
        return;
    }

    match a[1].as_str() {
        "day" => {
            run_day(&a[2])
        }
        _ => {
            print_usage();
        }
    }
}

fn run_day(day: &String)
{
    let result: Result<(String, Day), String> = parse_i32(day)
        .and_then(|d| read_input(d).and_then(|input| get_day(d).and_then(|day| Ok((input, day)))));
    match result {
        Ok((input, day)) => {
            (day.puzzle1)(&input);
            (day.puzzle2)(&input);
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}