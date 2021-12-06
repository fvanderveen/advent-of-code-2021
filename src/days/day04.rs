use crate::days::Day;
use crate::util::number;

pub const DAY4: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let puzzle = match parse_input(input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };

    let (winning_card, called_numbers) = match find_first_bingo(&puzzle) {
        Some(v) => v,
        None => panic!("Could not find a first bingo with the input?!")
    };

    let result = calculate_bingo_score(winning_card, called_numbers);
    println!("Puzzle 1 answer: {}", result);
}

fn puzzle2(input: &String) {
    let puzzle = match parse_input(input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };

    let (winning_card, called_numbers) = match find_last_bingo(&puzzle) {
        Some(v) => v,
        None => panic!("Could not find a first bingo with the input?!")
    };

    let result = calculate_bingo_score(winning_card, called_numbers);
    println!("Puzzle 2 answer: {}", result);
}

const BINGO_SIZE: usize = 5;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct BingoCard {
    cells: [[Option<Cell>; BINGO_SIZE]; BINGO_SIZE],
}

trait Bingo {
    fn has_bingo(&self, called_numbers: &[u128]) -> bool;
}

impl Bingo for BingoCard {
    fn has_bingo(&self, called_numbers: &[u128]) -> bool {
        // A card has bingo if a row or column is fully marked.
        for row in 0..BINGO_SIZE {
            if self.cells[row].iter().all(|cell| cell.is_marked(called_numbers)) {
                return true;
            }
        }
        for column in 0..BINGO_SIZE {
            if self.cells.iter().map(|r| &r[column]).all(|cell| cell.is_marked(called_numbers)) {
                return true;
            }
        }

        return false;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Cell {
    value: u128,
}

trait IsMarked {
    fn is_marked(&self, called_numbers: &[u128]) -> bool;
}

impl IsMarked for Cell {
    fn is_marked(&self, called_numbers: &[u128]) -> bool {
        called_numbers.contains(&self.value)
    }
}

impl IsMarked for Option<Cell> {
    fn is_marked(&self, called_numbers: &[u128]) -> bool {
        match self {
            Some(cell) => cell.is_marked(called_numbers),
            None => false
        }
    }
}

fn parse_bingo_card(input: &str) -> Result<BingoCard, String> {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    if lines.len() != BINGO_SIZE {
        return Err(format!("Expected {} rows, but got {} instead.", BINGO_SIZE, lines.len()));
    }

    let mut cells: [[Option<Cell>; BINGO_SIZE]; BINGO_SIZE] = [[None; BINGO_SIZE]; BINGO_SIZE];
    for i in 0..BINGO_SIZE {
        let line = lines[i];
        let entries: Vec<&str> = line.split(' ').filter(|e| !e.is_empty()).collect();
        if entries.len() != BINGO_SIZE {
            return Err(format!("Expected {} entries, but got {} instead in '{}' at index {}", BINGO_SIZE, entries.len(), line, i));
        }

        for j in 0..BINGO_SIZE {
            let entry = entries[j];
            match number::parse_u128(entry) {
                Ok(value) => { cells[i][j] = Some(Cell { value }) }
                Err(e) => return Err(e)
            }
        }
    }

    return Ok(BingoCard { cells });
}

fn parse_bingo_cards(input: &str) -> Result<Vec<BingoCard>, String> {
    input.split("\n\n").into_iter().map(|chunk| parse_bingo_card(chunk)).collect()
}

fn parse_called_numbers(input: &str) -> Result<Vec<u128>, String> {
    input.split(',').into_iter().map(|n| number::parse_u128(n)).collect()
}

fn find_first_bingo(input: &PuzzleInput) -> Option<(&BingoCard, &[u128])> {
    // Start at BINGO_SIZE, as that would be the least amount of numbers needed for a bingo.
    for i in BINGO_SIZE..input.called_numbers.len() {
        let current = &input.called_numbers[0..i];
        for card in &input.bingo_cards {
            if card.has_bingo(current) {
                return Some((card, current))
            }
        }
    }

    return None;
}

fn find_last_bingo(input: &PuzzleInput) -> Option<(&BingoCard, &[u128])> {
    let mut cards_remaining: Vec<&BingoCard> = input.bingo_cards.iter().collect();

    // Start at BINGO_SIZE, as that would be the least amount of numbers needed for a bingo.
    for i in BINGO_SIZE..input.called_numbers.len() {
        let current = &input.called_numbers[0..i];

        if cards_remaining.len() == 1 && cards_remaining[0].has_bingo(current) {
            return Some((&cards_remaining[0], current))
        }

        cards_remaining = cards_remaining.into_iter().filter(|c| !c.has_bingo(current)).collect();
    }

    return None;
}

fn calculate_bingo_score(bingo_card: &BingoCard, called_numbers: &[u128]) -> u128 {
    // Bingo score is calculated by summing all non-marked numbers, multiplied by the last called number
    let value: u128 = bingo_card.cells.iter().flat_map(|r| r.iter().map(|c| match c { Some(v) if !v.is_marked(called_numbers) => v.value, _ => 0 })).sum();
    let multiplier = called_numbers[called_numbers.len() - 1];

    return value * multiplier;
}

struct PuzzleInput {
    called_numbers: Vec<u128>,
    bingo_cards: Vec<BingoCard>,
}

fn parse_input(input: &str) -> Result<PuzzleInput, String> {
    let lines: Vec<&str> = input.lines().collect();
    let called_numbers = match parse_called_numbers(lines[0]) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let bingo_cards = match parse_bingo_cards(lines[2..].join("\n").as_str()) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };

    return Ok(PuzzleInput { called_numbers, bingo_cards });
}

#[cfg(test)]
mod tests {
    use crate::days::day04::{Bingo, BingoCard, calculate_bingo_score, Cell, find_first_bingo, find_last_bingo, IsMarked, parse_bingo_card, parse_called_numbers, parse_input};

    const EXAMPLE_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
        \n\
        22 13 17 11  0\n\
         8  2 23  4 24\n\
        21  9 14 16  7\n\
         6 10  3 18  5\n\
         1 12 20 15 19\n\
        \n\
         3 15  0  2 22\n\
         9 18 13 17  5\n\
        19  8  7 25 23\n\
        20 11 10 24  4\n\
        14 21 16 12  6\n\
        \n\
        14 21 17 24  4\n\
        10 16 15  9 19\n\
        18  8 23 26 20\n\
        22 11 13  6  5\n\
         2  0 12  3  7\n\
    ";

    const TEST_BINGO_CARD: BingoCard = BingoCard {
        cells: [
            [Some(Cell { value: 22 }), Some(Cell { value: 13 }), Some(Cell { value: 17 }), Some(Cell { value: 11 }), Some(Cell { value: 0 })],
            [Some(Cell { value: 8 }), Some(Cell { value: 2 }), Some(Cell { value: 23 }), Some(Cell { value: 4 }), Some(Cell { value: 24 })],
            [Some(Cell { value: 21 }), Some(Cell { value: 9 }), Some(Cell { value: 14 }), Some(Cell { value: 16 }), Some(Cell { value: 7 })],
            [Some(Cell { value: 6 }), Some(Cell { value: 10 }), Some(Cell { value: 3 }), Some(Cell { value: 18 }), Some(Cell { value: 5 })],
            [Some(Cell { value: 1 }), Some(Cell { value: 12 }), Some(Cell { value: 20 }), Some(Cell { value: 15 }), Some(Cell { value: 19 })]
        ]
    };

    #[test]
    fn test_is_marked() {
        let called_numbers = vec![12, 31, 1, 22, 98];
        assert_eq!(Cell { value: 1 }.is_marked(&called_numbers), true);
        assert_eq!(Cell { value: 31 }.is_marked(&called_numbers), true);
        assert_eq!(Cell { value: 32 }.is_marked(&called_numbers), false);
    }

    #[test]
    fn test_has_bingo() {
        assert_eq!(TEST_BINGO_CARD.has_bingo(&vec![]), false);
        assert_eq!(TEST_BINGO_CARD.has_bingo(&vec![22, 13, 17, 8, 9, 14, 11, 0]), true);
    }

    #[test]
    fn test_parse_called_numbers() {
        assert_eq!(parse_called_numbers("1,34,21,76,42,98"), Ok(vec![1, 34, 21, 76, 42, 98]));
        assert_eq!(parse_called_numbers("1,34,21,a,42,98").is_err(), true);
    }

    #[test]
    fn test_parse_bingo_card() {
        let card = parse_bingo_card("\
            22 13 17 11  0\n\
             8  2 23  4 24\n\
            21  9 14 16  7\n\
             6 10  3 18  5\n\
             1 12 20 15 19\n\
        ");

        assert_eq!(card, Ok(TEST_BINGO_CARD));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(EXAMPLE_INPUT);
        // The called methods are tested separately, just verify this yields OK
        assert!(result.is_ok());
    }

    #[test]
    fn test_find_first_bingo() {
        let input = parse_input(EXAMPLE_INPUT).unwrap();
        let result = find_first_bingo(&input);

        assert_eq!(result, Some((
            &input.bingo_cards[2],
            &[7,4,9,5,11,17,23,2,0,14,21,24][..]
        )));
    }

    #[test]
    fn test_find_last_bingo() {
        let input = parse_input(EXAMPLE_INPUT).unwrap();
        let result = find_last_bingo(&input);

        assert_eq!(result, Some((
            &input.bingo_cards[1],
            &[7,4,9,5,11,17,23,2,0,14,21,24,10,16,13][..]
        )));
    }

    #[test]
    fn test_calculate_bingo_score() {
        let input = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(calculate_bingo_score(&input.bingo_cards[2], &[7,4,9,5,11,17,23,2,0,14,21,24][..]), 4512)
    }
}