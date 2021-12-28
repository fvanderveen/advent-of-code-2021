use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Grid, Point};

pub const DAY25: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let mut grid: Grid<Snail> = input.parse().unwrap();
    
    let result = grid.cycle_till_stacked();
    
    println!("Puzzle 1 answer: {}", result);
}

fn puzzle2(_input: &String) {
    println!("Puzzle 2 is a freebie, as always :D");
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Snail {
    None,
    East,
    South,
}

impl std::fmt::Display for Snail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Snail::None => '.',
            Snail::East => '>',
            Snail::South => 'v'
        })
    }
}

impl FromStr for Snail {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Snail::None),
            ">" => Ok(Snail::East),
            "v" => Ok(Snail::South),
            _ => Err(format!("Not a snail: '{}'", s))
        }
    }
}

impl Default for Snail {
    fn default() -> Self {
        Snail::None
    }
}

impl Snail {
    fn get_next_position(&self, grid: &Grid<Snail>, current: &Point) -> Option<Point> {
        let mut target = current.clone();

        match self {
            Snail::None => { return None }
            Snail::East => {
                target.x += 1;
                if target.x == grid.bounds.right() {
                    target.x = grid.bounds.left;
                }
            }
            Snail::South => {
                target.y += 1;
                if target.y == grid.bounds.bottom() {
                    target.y = grid.bounds.top;
                }
            }
        }

        if let Some(s) = grid.get(&target) {
            if s == Snail::None {
                Some(target)
            } else {
                None
            }
        } else {
            Some(target)
        }
    }
}

impl Grid<Snail> {
    fn cycle(&mut self) -> bool {
        fn move_snails(snails: Vec<(Point, Snail)>, grid: &mut Grid<Snail>) -> bool {
            let mut changed = false;
            // Since all snails move in one go; we need to keep the old grid for checking next positions:
            let state = grid.clone();
            for (p, s) in snails {
                if let Some(t) = s.get_next_position(&state, &p) {
                    grid.set(p.clone(), Snail::None);
                    grid.set(t, s.clone());
                    changed = true;
                }
            }
            changed
        }

        let mut changed = false;
        let snails: Vec<_> = self.entries();
        let east_snails: Vec<_> = snails.iter().filter(|(_, s)| Snail::East.eq(s)).cloned().collect();
        changed |= move_snails(east_snails, self);
        let south_snails: Vec<_> = snails.iter().filter(|(_, s)| Snail::South.eq(s)).cloned().collect();
        changed |= move_snails(south_snails, self);
        changed
    }
    
    fn cycle_till_stacked(&mut self) -> usize {
        let mut steps = 1;
        while self.cycle() {
            steps += 1
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day25::Snail;
    use crate::util::geometry::Grid;

    const EXAMPLE_INPUT: &str = "\
        v...>>.vv>\n\
        .vv>>.vv..\n\
        >>.>v>...v\n\
        >>v>>.>.v.\n\
        v>v.vv.v..\n\
        >.>>..v...\n\
        .vv..>.>v.\n\
        v.v..>>v.v\n\
        ....v..v.>";

    #[test]
    fn test_cycle() {
        let mut grid: Grid<Snail> = EXAMPLE_INPUT.parse().unwrap();
        assert_eq!(grid.cycle(), true);
        assert_eq!(format!("{}", grid), "\
            ....>.>v.>\n\
            v.v>.>v.v.\n\
            >v>>..>v..\n\
            >>v>v>.>.v\n\
            .>v.v...v.\n\
            v>>.>vvv..\n\
            ..v...>>..\n\
            vv...>>vv.\n\
            >.v.v..v.v");
        assert_eq!(grid.cycle(), true);
        assert_eq!(format!("{}", grid), "\
            >.v.v>>..v\n\
            v.v.>>vv..\n\
            >v>.>.>.v.\n\
            >>v>v.>v>.\n\
            .>..v....v\n\
            .>v>>.v.v.\n\
            v....v>v>.\n\
            .vv..>>v..\n\
            v>.....vv.");
    }
    
    #[test]
    fn test_cycle_till_stacked() {
        let mut grid: Grid<Snail> = EXAMPLE_INPUT.parse().unwrap();
        assert_eq!(grid.cycle_till_stacked(), 58);
    }
}