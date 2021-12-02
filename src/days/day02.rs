use crate::days::Day;
use crate::util::number;

pub const DAY2: Day = Day {
    puzzle1,
    puzzle2
};

enum Command {
    FORWARD,
    UP,
    DOWN
}

struct Instruction {
    command: Command,
    value: i32
}

fn parse_instruction(input: &str) -> Result<Instruction, String> {
    let parts: Vec<&str> = input.split(" ").collect();
    if parts.len() != 2 {
        return Err(format!("Invalid instruction: {}", input));
    }

    let value = match number::parse_i32(parts[1]) {
        Ok(v) => { v },
        Err(e) => { return Err(format!("Invalid value in instruction {}: {}", input, e)) }
    };

    match parts[0] {
        "forward" => {
            Ok(Instruction { command: Command::FORWARD, value })
        }
        "up" => {
            Ok(Instruction { command: Command::UP, value })
        }
        "down" => {
            Ok(Instruction { command: Command::DOWN, value })
        }
        comm => Err(format!("Invalid command: {}", comm))
    }
}

fn to_instructions(input: &str) -> Result<Vec<Instruction>, String> {
    input.lines().map(parse_instruction).collect()
}

fn puzzle1(input: &String) {
    let instructions = match to_instructions(input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };

    let mut depth = 0;
    let mut distance = 0;

    for instruction in instructions {
        match instruction.command {
            Command::FORWARD => { distance += instruction.value }
            Command::UP => { depth -= instruction.value }
            Command::DOWN => { depth += instruction.value }
        }
    }

    let result = depth * distance;
    println!("Puzzle 1 result: {}", result);
}
fn puzzle2(input: &String) {
    let instructions = match to_instructions(input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };

    let mut aim: i32 = 0;
    let mut depth: i128 = 0;
    let mut distance: u128 = 0;

    for instruction in instructions {
        match instruction.command {
            Command::FORWARD => { distance += instruction.value as u128; depth += (aim * instruction.value) as i128 }
            Command::UP => { aim -= instruction.value }
            Command::DOWN => { aim += instruction.value }
        }
    }

    let result = depth * distance as i128;
    println!("Puzzle 2 result: {}", result);
}