use std::collections::HashMap;
use crate::days::Day;
use crate::days::day10::ErrorType::Incomplete;

pub const DAY10: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let error_score: u64 = input.lines().filter_map(|l| check_line(l)).map(|e| get_error_score(e)).sum();

    println!("Puzzle 1 answer: {}", error_score);
}

fn puzzle2(input: &String) {
    let mut completion_scores: Vec<u64> = input.lines().filter_map(|l| check_line(l)).filter_map(|e| get_completion_score(e)).collect();
    completion_scores.sort();
    let result = completion_scores[completion_scores.len()/2];

    println!("Puzzle 2 answer: {}", result);
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum ErrorType {
    Incomplete,
    // Missing characters at the end
    Corrupt,
    // Mismatched closing character
    Invalid, // Invalid character
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct SyntaxError {
    index: usize,
    invalid: Option<char>,
    error_type: ErrorType,
    expected: Option<String>,
}

fn check_line(line: &str) -> Option<SyntaxError> {
    // The allowed syntax here is to have pairs of characters nested pretty much like HTML or code in general
    // [], (), <>, and {} are the valid pairs
    // [], [()], and {{([(())])}} are examples of valid inputs.
    // A line can contain multiple (nested) pairs

    let match_table: HashMap<char, char> = HashMap::from([
        ('[', ']'), (']', '['),
        ('{', '}'), ('}', '{'),
        ('(', ')'), (')', '('),
        ('<', '>'), ('>', '<')
    ]);

    let mut stack: Vec<char> = vec![];
    let chars: Vec<char> = line.chars().collect();

    for i in 0..chars.len() {
        let current = chars[i];
        match current {
            '[' | '{' | '<' | '(' => stack.push(current),
            ']' | '}' | '>' | ')' => {
                let expected_char = match_table.get(&current).unwrap(); // safe unwrap due to match
                let actual = stack.pop();
                match actual {
                    // If there is no character on the stack, we would expect the matching open character to be inserted as fix.
                    None => return Some(SyntaxError { index: i, invalid: Some(current), error_type: ErrorType::Corrupt, expected: Some(format!("{}", expected_char)) }),
                    Some(c) if !c.eq(expected_char) => {
                        // We don't have the right character on the stack, figure out what extra closings need to be inserted for the current to be valid
                        let mut expected: String = String::from(match_table.get(&c).unwrap().clone());
                        loop {
                            match stack.pop() {
                                None => {
                                    // Stack is empty, but we didn't fix the issue yet. We'll need to also insert the expected character:
                                    expected += String::from(expected_char.clone()).as_str();
                                    return Some(SyntaxError { index: i, invalid: Some(current), error_type: ErrorType::Corrupt, expected: Some(expected) });
                                }
                                Some(c) if c.eq(expected_char) => {
                                    // We're at the point in the stack where the current char would be valid
                                    return Some(SyntaxError { index: i, invalid: Some(current), error_type: ErrorType::Corrupt, expected: Some(expected) });
                                }
                                Some(c) => {
                                    // Still the wrong character, we need to close another group first.
                                    expected += String::from(match_table.get(&c).unwrap().clone()).as_str();
                                }
                            }
                        }
                    }
                    _ => { /* we just got the expected character, move on */ }
                }
            }
            _ => {
                return Some(SyntaxError { index: i, invalid: Some(current), error_type: ErrorType::Invalid, expected: None });
            }
        }
    }

    if stack.is_empty() {
        None
    } else {
        let expected: String = stack.iter().rev().map(|c| format!("{}", match_table.get(c).unwrap())).collect::<String>();
        Some(SyntaxError { index: line.len(), invalid: None, error_type: ErrorType::Incomplete, expected: Some(expected) })
    }
}

fn get_error_score(error: SyntaxError) -> u64 {
    match error {
        SyntaxError { index: _, invalid: Some(c), error_type: ErrorType::Corrupt, expected: _ } => {
            match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0
            }
        }
        _ => 0
    }
}

fn get_completion_score(error: SyntaxError) -> Option<u64> {
    if error.error_type != Incomplete {
        return None;
    }

    error.expected.and_then(|e| e.chars().map(|c| match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0
    }).reduce(|a, b| a * 5 + b))
}

#[cfg(test)]
mod tests {
    use crate::days::day10::{check_line, get_completion_score, get_error_score, SyntaxError};
    use crate::days::day10::ErrorType::{Corrupt, Incomplete};

    const EXAMPLE_INPUT: &str = "\
        [({(<(())[]>[[{[]{<()<>>\n\
        [(()[<>])]({[<{<<[]>>(\n\
        {([(<{}[<>[]}>{[]{[(<()>\n\
        (((({<>}<{<{<>}{[]{[]{}\n\
        [[<[([]))<([[{}[[()]]]\n\
        [{[{({}]{}}([{[{{{}}([]\n\
        {<[[]]>}<{[{[{[]{()[[[]\n\
        [<(<(<(<{}))><([]([]()\n\
        <{([([[(<>()){}]>(<<{{\n\
        <{([{{}}[<[[[<>{}]]]>[]]\
    ";

    #[test]
    fn test_check_line() {
        assert_eq!(check_line("[]"), None);
        assert_eq!(check_line("[](){}<>"), None);
        assert_eq!(check_line("[<<<()()>>>]"), None);

        assert_eq!(check_line("[)"), Some(SyntaxError { index: 1, invalid: Some(')'), error_type: Corrupt, expected: Some(String::from("](")) }));
        assert_eq!(check_line("<<[[()]>>"), Some(SyntaxError { index: 7, invalid: Some('>'), error_type: Corrupt, expected: Some(String::from("]")) }));

        assert_eq!(check_line("<<[[()]]>"), Some(SyntaxError { index: 9, invalid: None, error_type: Incomplete, expected: Some(String::from(">")) }));
        assert_eq!(check_line("<<[[({{("), Some(SyntaxError { index: 8, invalid: None, error_type: Incomplete, expected: Some(String::from(")}})]]>>")) }));
    }

    #[test]
    fn test_error_score() {
        assert_eq!(get_error_score(SyntaxError { index: 9, invalid: Some(')'), error_type: Corrupt, expected: None }), 3);
        assert_eq!(get_error_score(SyntaxError { index: 9, invalid: Some(')'), error_type: Incomplete, expected: None }), 0);
        assert_eq!(get_error_score(SyntaxError { index: 0, invalid: Some(']'), error_type: Corrupt, expected: None }), 57);
        assert_eq!(get_error_score(SyntaxError { index: 100, invalid: Some('}'), error_type: Corrupt, expected: None }), 1197);
        assert_eq!(get_error_score(SyntaxError { index: 97, invalid: Some('>'), error_type: Corrupt, expected: None }), 25137);
    }

    #[test]
    fn test_example_input_corrupt_score() {
        let lines: Vec<&str> = EXAMPLE_INPUT.lines().collect();

        let score: u64 = lines.into_iter().filter_map(|l| check_line(l)).map(|e| get_error_score(e)).sum();
        assert_eq!(score, 26397);
    }

    #[test]
    fn test_example_input_completion_score() {
        let mut scores: Vec<u64> = EXAMPLE_INPUT.lines().filter_map(|l| check_line(l)).filter_map(|e| get_completion_score(e)).collect();
        scores.sort();
        let result = scores[scores.len() / 2];
        assert_eq!(result, 288957)
    }
}