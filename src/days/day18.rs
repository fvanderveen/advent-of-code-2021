use crate::days::Day;

pub const DAY18: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let numbers = parse_puzzle_input(input).unwrap();

    let result = sum_list(&numbers).unwrap().magnitude();

    println!("Puzzle 1 answer: {}", result);
}

fn puzzle2(input: &String) {
    let numbers = parse_puzzle_input(input).unwrap();

    let max_magnitude = numbers.iter().flat_map(|x|
        numbers.iter().filter(|y| x.ne(y)).map(|y| x.add(y).reduce().magnitude())
    ).max().unwrap();

    println!("Puzzle 2 answer: {}", max_magnitude);
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct SnailEntry {
    value: usize,
    level: usize,
}

#[derive(Eq, PartialEq, Clone)]
struct SnailNumber {
    entries: Vec<SnailEntry>,
}

impl SnailNumber {
    fn add(&self, other: &SnailNumber) -> SnailNumber {
        SnailNumber {
            entries: self.entries.iter()
                .chain(other.entries.iter())
                .map(|e| SnailEntry { value: e.value, level: e.level + 1 })
                .collect()
        }
    }

    fn reduce(&self) -> SnailNumber {
        let mut result = self.clone();

        // Each step:
        // 1. Find the first pair with a level of 5, if any, explode it
        // 2. Find the first value >= 10, if any, split it
        // repeat until nothing done.

        loop {
            // Handle explosion
            if let Some(index) = result.entries.iter().position(|e| e.level == 5) {
                let left = result.entries[index];
                let right = result.entries[index + 1]; // Panics if not exists; but should exist, so eh.
                // Add left to the previous value, if any:
                if index > 0 {
                    result.entries[index - 1].value = result.entries[index - 1].value + left.value;
                }
                // Add right the the next value, if any:
                if let Some(entry) = result.entries.get_mut(index + 2) {
                    entry.value = entry.value + right.value;
                }
                // Remove the old entries:
                result.entries.remove(index + 1);
                result.entries.remove(index);
                // Add a new, flat '0' entry instead of the pair.
                result.entries.insert(index, SnailEntry { value: 0, level: 4 });
                continue; // Next loop.
            }

            // Handle split:
            if let Some(index) = result.entries.iter().position(|e| e.value >= 10) {
                let entry = result.entries[index];

                let left = entry.value / 2; // Rounds down
                let right = entry.value - left; // The other half (rounds up)

                result.entries.remove(index);
                result.entries.insert(index, SnailEntry { value: right, level: entry.level + 1 });
                result.entries.insert(index, SnailEntry { value: left, level: entry.level + 1 });

                continue;
            }

            // If neither got handled, we're done.
            break;
        }

        result
    }

    fn magnitude(&self) -> usize {
        fn handle_value(current_index: &mut usize, level: usize, list: &Vec<SnailEntry>) -> usize {
            list.get(*current_index).map(|e| {
                if e.level == level {
                    *current_index += 1;
                    e.value
                } else if e.level > level {
                    handle_level(current_index, level + 1, list)
                } else {
                    0
                }
            }).unwrap_or(0)
        }

        fn handle_level(current_index: &mut usize, level: usize, list: &Vec<SnailEntry>) -> usize {
            let left = handle_value(current_index, level, list);
            let right = handle_value(current_index, level, list);

            return 3 * left + 2 * right;
        }

        handle_level(&mut 0, 1, &self.entries)
    }
}

impl From<(usize, usize)> for SnailEntry {
    fn from(tuple: (usize, usize)) -> Self {
        SnailEntry { value: tuple.0, level: tuple.1 }
    }
}

impl From<Vec<(usize, usize)>> for SnailNumber {
    fn from(vec: Vec<(usize, usize)>) -> Self {
        SnailNumber { entries: vec.iter().cloned().map(|t| t.into()).collect() }
    }
}

impl std::str::FromStr for SnailNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let mut current_level = 0;
        let mut entries = vec![];

        for char in chars {
            match char {
                '[' => current_level += 1,
                ']' => current_level -= 1,
                c if c.is_digit(10) => {
                    entries.push(SnailEntry {
                        value: c.to_digit(10).ok_or(format!("Invalid token '{}' in SnailNumber", c)).and_then(|v| v.try_into().map_err(|e| format!("{}", e)))?,
                        level: current_level,
                    });
                }
                ',' => { /* ignore */ }
                inv => return Err(format!("Invalid token '{}' in SnailNumber", inv))
            }
        }

        Ok(SnailNumber { entries })
    }
}

impl std::fmt::Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn handle_value(current_index: &mut usize, level: usize, items: &Vec<SnailEntry>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if *current_index >= items.len() {
                return write!(f, "!!Unbalanced number!!");
            }

            let item = &items[*current_index];
            if item.level == level {
                // Direct value for us:
                *current_index += 1;
                write!(f, "{}", item.value)
            } else if item.level > level {
                // Deeper level, pass on
                handle_level(current_index, level + 1, items, f)
            } else {
                write!(f, "!!Missing value {} < {}!!", item.level, level)
            }
        }

        fn handle_level(current_index: &mut usize, level: usize, items: &Vec<SnailEntry>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            handle_value(current_index, level, items, f)?;
            write!(f, ",")?;
            handle_value(current_index, level, items, f)?;
            write!(f, "]")
        }

        handle_level(&mut 0, 1, &self.entries, f)
    }
}

impl std::fmt::Debug for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SnailNumber {{ {} }}", self)
    }
}

fn parse_puzzle_input(input: &str) -> Result<Vec<SnailNumber>, String> {
    input.lines().filter(|l| !l.is_empty()).map(|l| l.parse()).collect()
}

fn sum_list(list: &Vec<SnailNumber>) -> Option<SnailNumber> {
    list.iter().cloned().reduce(|lhs, rhs| lhs.add(&rhs).reduce())
}

#[cfg(test)]
mod tests {
    use crate::days::day18::{parse_puzzle_input, SnailNumber, sum_list};

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", SnailNumber::from(vec![(1, 1), (2, 2), (3, 2)])), "[1,[2,3]]");
        assert_eq!(format!("{}", SnailNumber::from(vec![(1, 2), (2, 2), (3, 2), (4, 2)])), "[[1,2],[3,4]]");
    }

    #[test]
    fn test_parse() {
        assert_eq!("[1,[2,3]]".parse(), Ok(SnailNumber::from(vec![(1, 1), (2, 2), (3, 2)])));
    }

    #[test]
    fn test_parse_display() {
        // Some other samples we simply test by parse -> display to ensure they end up the same.
        assert_eq!(format!("{}", "[1,2]".parse::<SnailNumber>().unwrap()), "[1,2]");
        assert_eq!(format!("{}", "[[1,2],3]".parse::<SnailNumber>().unwrap()), "[[1,2],3]");
        assert_eq!(format!("{}", "[9,[8,7]]".parse::<SnailNumber>().unwrap()), "[9,[8,7]]");
        assert_eq!(format!("{}", "[[1,9],[8,5]]".parse::<SnailNumber>().unwrap()), "[[1,9],[8,5]]");
        assert_eq!(format!("{}", "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]".parse::<SnailNumber>().unwrap()), "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
        assert_eq!(format!("{}", "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]".parse::<SnailNumber>().unwrap()), "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        assert_eq!(format!("{}", "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]".parse::<SnailNumber>().unwrap()), "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    }

    #[test]
    fn test_add() {
        let l1: SnailNumber = "[1,2]".parse().unwrap();
        let r1: SnailNumber = "[[1,2],3]".parse().unwrap();
        assert_eq!(format!("{}", l1.add(&r1)), "[[1,2],[[1,2],3]]");
        assert_eq!(format!("{}", r1.add(&l1)), "[[[1,2],3],[1,2]]");
    }

    #[test]
    fn test_reduce() {
        let split = SnailNumber { entries: vec![(1, 1).into(), (2, 2).into(), (13, 2).into()] };
        let reduced = split.reduce();
        assert_eq!(format!("{}", reduced), "[1,[2,[6,7]]]");

        let explode1: SnailNumber = "[[[[[9,8],1],2],3],4]".parse().unwrap();
        assert_eq!(format!("{}", explode1.reduce()), "[[[[0,9],2],3],4]");
    }

    #[test]
    fn test_magnitude() {
        assert_eq!("[[1,2],[[3,4],5]]".parse::<SnailNumber>().unwrap().magnitude(), 143);
        assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse::<SnailNumber>().unwrap().magnitude(), 1384);
        assert_eq!("[[[[1,1],[2,2]],[3,3]],[4,4]]".parse::<SnailNumber>().unwrap().magnitude(), 445);
        assert_eq!("[[[[3,0],[5,3]],[4,4]],[5,5]]".parse::<SnailNumber>().unwrap().magnitude(), 791);
        assert_eq!("[[[[5,0],[7,4]],[5,5]],[6,6]]".parse::<SnailNumber>().unwrap().magnitude(), 1137);
        assert_eq!("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse::<SnailNumber>().unwrap().magnitude(), 3488);
    }

    #[test]
    fn itest_sum_list_reduce() {
        let e1: &str = "\
            [1,1]\n\
            [2,2]\n\
            [3,3]\n\
            [4,4]";
        let r1 = sum_list(&parse_puzzle_input(e1).unwrap()).unwrap();
        assert_eq!(format!("{}", r1), "[[[[1,1],[2,2]],[3,3]],[4,4]]");

        let e2: &str = "\
            [1,1]\n\
            [2,2]\n\
            [3,3]\n\
            [4,4]\n\
            [5,5]";
        let r2 = sum_list(&parse_puzzle_input(e2).unwrap()).unwrap();
        assert_eq!(format!("{}", r2), "[[[[3,0],[5,3]],[4,4]],[5,5]]");

        let e3: &str = "\
            [1,1]\n\
            [2,2]\n\
            [3,3]\n\
            [4,4]\n\
            [5,5]\n\
            [6,6]";
        let r3 = sum_list(&parse_puzzle_input(e3).unwrap()).unwrap();
        assert_eq!(format!("{}", r3), "[[[[5,0],[7,4]],[5,5]],[6,6]]");

        let e4: &str = "\
            [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n\
            [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n\
            [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n\
            [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n\
            [7,[5,[[3,8],[1,4]]]]\n\
            [[2,[2,2]],[8,[8,1]]]\n\
            [2,9]\n\
            [1,[[[9,3],9],[[9,0],[0,7]]]]\n\
            [[[5,[7,4]],7],1]\n\
            [[[[4,2],2],6],[8,7]]";
        let r4 = sum_list(&parse_puzzle_input(e4).unwrap()).unwrap();
        assert_eq!(format!("{}", r4), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }

    #[test]
    fn test_puzzle1_example() {
        let input: &str = "\
            [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n\
            [[[5,[2,8]],4],[5,[[9,9],0]]]\n\
            [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n\
            [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n\
            [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n\
            [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n\
            [[[[5,4],[7,7]],8],[[8,3],8]]\n\
            [[9,3],[[9,9],[6,[4,9]]]]\n\
            [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n\
            [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let nums = parse_puzzle_input(input).unwrap();
        let sum = sum_list(&nums).unwrap();
        assert_eq!(format!("{}", sum), "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
        assert_eq!(sum.magnitude(), 4140);
    }
}
