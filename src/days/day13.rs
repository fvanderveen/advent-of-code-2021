use std::fmt;
use std::str::FromStr;
use regex::Regex;
use crate::days::Day;
use crate::util::geometry::Point;
use crate::util::number;
use crate::util::collection::{CollectionExtension};

pub const DAY13: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let paper: Paper = input.parse().unwrap();

    let folded_once = paper.fold();

    println!("Puzzle 1 answer: {}", folded_once.dots.len());
}

fn puzzle2(input: &String) {
    let mut paper: Paper = input.parse().unwrap();

    while !paper.instructions.is_empty() {
        paper = paper.fold();
    }

    println!("Puzzle 2 answer:\n{}", paper);
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Paper {
    dots: Vec<Point>,
    instructions: Vec<FoldInstruction>,
}

impl Paper {
    /// Consume the top-most fold instruction, and return a new Paper representing the result.
    fn fold(&self) -> Paper {
        if self.instructions.is_empty() {
            return self.clone(); // Do make a clone, even though nothing changed.
        }

        let instruction = self.instructions[0];
        let instructions = self.instructions[1..].to_vec();

        let dots: Vec<Point> = self.dots.iter().map(|p| instruction.apply(p)).collect();
        Paper { dots: dots.deduplicate(), instructions }
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_x = self.dots.iter().map(|p| p.x).max().unwrap_or(0);
        let max_y = self.dots.iter().map(|p| p.y).max().unwrap_or(0);

        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.dots.contains(&(x, y).into()) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        if !self.instructions.is_empty() {
            write!(f, "\n")?;
            for instruction in &self.instructions {
                writeln!(f, "{}", instruction)?;
            }
        }

        Ok(())
    }
}

impl FromStr for Paper {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("\n\n").collect();
        if parts.len() != 2 { return Err(format!("Invalid format: {}", s)); }

        let dots: Vec<Point> = parts[0].lines().map(|l| l.parse()).collect::<Result<Vec<Point>, String>>()?;
        let instructions: Vec<FoldInstruction> = parts[1].lines().map(|l| l.parse()).collect::<Result<Vec<FoldInstruction>, String>>()?;

        Ok(Paper { dots, instructions })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum FoldAxis { X, Y }

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct FoldInstruction {
    axis: FoldAxis,
    value: usize,
}

impl FoldInstruction {
    /// Apply this instruction to the given point.
    fn apply(&self, point: &Point) -> Point {
        let fold: isize = self.value as isize;
        match self.axis {
            FoldAxis::X if point.x <= fold => Point { x: point.x, y: point.y },
            FoldAxis::X => Point { x: fold - (point.x - fold), y: point.y },
            FoldAxis::Y if point.y <= fold => Point { x: point.x, y: point.y },
            FoldAxis::Y => Point { x: point.x, y: fold - (point.y - fold) }
        }
    }
}

impl fmt::Display for FoldInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let axis: &str = match self.axis {
            FoldAxis::X => "x",
            FoldAxis::Y => "y"
        };
        write!(f, "fold along {}={}", axis, self.value)
    }
}

impl FromStr for FoldInstruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = match Regex::new("^fold along ([xy])=(\\d+)$") {
            Ok(r) => r,
            Err(e) => return Err(format!("Could build regex?! {}", e))
        };

        let captures = match regex.captures(s) {
            Some(c) => c,
            None => return Err(format!("Invalid fold instruction: {}", s))
        };
        let axis = match captures.get(1) {
            Some(v) if v.as_str() == "x" => FoldAxis::X,
            Some(v) if v.as_str() == "y" => FoldAxis::Y,
            Some(v) => return Err(format!("Invalid fold axis {} in {}", v.as_str(), s)),
            None => return Err(format!("Invalid fold instruction: {}", s))
        };
        let value = match captures.get(2).map(|v| number::parse_usize(v.as_str())) {
            Some(Ok(v)) => v,
            Some(Err(e)) => return Err(e),
            None => return Err(format!("Invalid fold instruction: {}", s))
        };
        Ok(FoldInstruction { axis, value })
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day13::{FoldInstruction, Paper};
    use crate::days::day13::FoldAxis::{X, Y};

    const EXAMPLE_INPUT: &str = "\
        6,10\n\
        0,14\n\
        9,10\n\
        0,3\n\
        10,4\n\
        4,11\n\
        6,0\n\
        6,12\n\
        4,1\n\
        0,13\n\
        10,12\n\
        3,4\n\
        3,0\n\
        8,4\n\
        1,10\n\
        2,14\n\
        8,10\n\
        9,0\n\
        \n\
        fold along y=7\n\
        fold along x=5\
    ";

    #[test]
    fn test_format() {
        let paper: Paper = Paper {
            dots: vec![(0, 1).into(), (2, 2).into(), (3, 0).into(), (1, 3).into()],
            instructions: vec![FoldInstruction { axis: X, value: 2 }],
        };

        assert_eq!(format!("{}", paper), "\
            ...#\n\
            #...\n\
            ..#.\n\
            .#..\n\
            \n\
            fold along x=2\n\
        ")
    }

    #[test]
    fn test_parse() {
        let paper: Result<Paper, String> = EXAMPLE_INPUT.parse();
        assert_eq!(paper, Ok(Paper {
            dots: vec![
                (6, 10).into(),
                (0, 14).into(),
                (9, 10).into(),
                (0, 3).into(),
                (10, 4).into(),
                (4, 11).into(),
                (6, 0).into(),
                (6, 12).into(),
                (4, 1).into(),
                (0, 13).into(),
                (10, 12).into(),
                (3, 4).into(),
                (3, 0).into(),
                (8, 4).into(),
                (1, 10).into(),
                (2, 14).into(),
                (8, 10).into(),
                (9, 0).into(),
            ],
            instructions: vec![
                FoldInstruction { axis: Y, value: 7 },
                FoldInstruction { axis: X, value: 5 },
            ],
        }))
    }

    #[test]
    fn test_fold() {
        let paper: Paper = EXAMPLE_INPUT.parse().unwrap();
        let folded = paper.fold();

        assert_eq!(format!("{}", folded), "\
            #.##..#..#.\n\
            #...#......\n\
            ......#...#\n\
            #...#......\n\
            .#.#..#.###\n\
            \n\
            fold along x=5\n\
        ");
        assert_eq!(folded.dots.len(), 17);

        let folded_twice = folded.fold();
        assert_eq!(format!("{}", folded_twice), "\
            #####\n\
            #...#\n\
            #...#\n\
            #...#\n\
            #####\n\
        ");
        assert_eq!(folded_twice.dots.len(), 16);
    }
}
