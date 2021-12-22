use std::collections::HashMap;
use std::hash::Hash;
use std::ops::RangeInclusive;
use std::str::FromStr;
use crate::days::Day;

pub const DAY20: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let mut puzzle: Puzzle = input.parse().unwrap();

    puzzle.enhance();
    puzzle.enhance();

    println!("Puzzle 1 answer: {}", puzzle.get_lit_pixels());
}

fn puzzle2(input: &String) {
    let mut puzzle: Puzzle = input.parse().unwrap();

    for _ in 0..50 {
        puzzle.enhance();
    }

    println!("Puzzle 2 answer: {}", puzzle.get_lit_pixels());
}

// We need to represent an infinity grid. We're only interested in the ones with a value and those
// directly around them though. Probably a map would just suffice.

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Location {
    x: isize,
    y: isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Bounds {
    top: isize,
    left: isize,
    width: isize,
    height: isize,
}

impl Bounds {
    fn grow(&mut self, by: isize) {
        self.top -= by;
        self.left -= by;
        self.width += 2 * by;
        self.height += 2 * by;
    }

    fn y(&self) -> RangeInclusive<isize> {
        self.top..=self.top + self.height
    }

    fn x(&self) -> RangeInclusive<isize> {
        self.left..=self.left + self.width
    }

    fn contains(&self, pixel: &Location) -> bool {
        self.x().contains(&pixel.x) && self.y().contains(&pixel.y)
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Image {
    pixels: HashMap<Location, bool>,
    outer_value: bool,
    bounds: Bounds,
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bounds = self.bounds;
        bounds.grow(2);

        // Print, to make the image better understandable, we include 2 pixels around the map limits.
        for y in bounds.y() {
            for x in bounds.x() {
                write!(f, "{}", match self.get_pixel(&Location { x, y }) {
                    true => '#',
                    false => '.'
                })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image {{\n{}}}", self)
    }
}

impl Image {
    fn new(pixels: HashMap<Location, bool>, outer_value: bool) -> Self {
        let locations: Vec<&Location> = pixels.keys().collect();
        let min_x = locations.iter().map(|l| l.x).min().unwrap_or(0);
        let max_x = locations.iter().map(|l| l.x).max().unwrap_or(0);
        let min_y = locations.iter().map(|l| l.y).min().unwrap_or(0);
        let max_y = locations.iter().map(|l| l.y).max().unwrap_or(0);

        Image {
            pixels,
            outer_value,
            bounds: Bounds {
                top: min_y,
                left: min_x,
                width: max_x - min_x,
                height: max_y - min_y,
            },
        }
    }

    fn enhance(&self, enhancement: &[bool; 512]) -> Self {
        let mut pixels = HashMap::new();
        // The new outer value is either 000000000 (=0) or 111111111 = (511)
        let outer_value = if self.outer_value { enhancement[511] } else { enhancement[0] };

        // For all current pixels calculate the enhanced value
        let mut bounds = self.bounds;
        bounds.grow(1);

        for y in bounds.y() {
            for x in bounds.x() {
                let value = self.get_value(&Location { x, y });
                pixels.insert(Location { x, y }, enhancement[value]);
            }
        }

        Image::new(pixels, outer_value)
    }

    fn get_value(&self, pixel: &Location) -> usize {
        let mut result = 0;

        for y in pixel.y - 1..=pixel.y + 1 {
            for x in pixel.x - 1..=pixel.x + 1 {
                result <<= 1;
                result += match self.get_pixel(&Location { x, y }) {
                    true => 1,
                    false => 0
                };
            }
        }

        result
    }

    fn get_pixel(&self, pixel: &Location) -> bool {
        if !self.bounds.contains(pixel) {
            self.outer_value
        } else {
            self.pixels.get(pixel).cloned().unwrap_or(false)
        }
    }

    fn get_lit_pixels(&self) -> usize {
        self.pixels.values().filter(|v| true.eq(*v)).count()
    }
}


#[derive(Eq, PartialEq, Clone, Debug)]
struct Puzzle {
    enhancement: [bool; 512],
    image: Image,
}

impl FromStr for Puzzle {
    type Err = String;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = data.lines().collect();

        let raw_enhancement: [char; 512] = lines[0].chars().collect::<Vec<char>>().try_into().map_err(|e: Vec<char>| format!("Enhancement data wrong length: {}", e.len()))?;
        let enhancement: [bool; 512] = raw_enhancement.map(|c| match c {
            '#' => true,
            _ => false
        });

        let grid = lines.iter().skip(2)
            .map(|l| l.chars().map(|c| match c {
                '.' => Ok(false),
                '#' => Ok(true),
                _ => Err("Invalid character in image".to_owned())
            }).collect::<Result<Vec<bool>, String>>())
            .collect::<Result<Vec<Vec<bool>>, String>>()?;

        let mut pixels = HashMap::new();

        for y in 0..grid.len() {
            let line = &grid[y];
            for x in 0..line.len() {
                pixels.insert(Location { x: x as isize, y: y as isize }, line[x]);
            }
        }

        Ok(Puzzle { enhancement, image: Image::new(pixels, false) })
    }
}

impl Puzzle {
    fn enhance(&mut self) {
        self.image = self.image.enhance(&self.enhancement);
    }

    fn get_lit_pixels(&self) -> usize {
        self.image.get_lit_pixels()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day20::Puzzle;

    #[test]
    fn test_parse_data() {
        let puzzle: Result<Puzzle, String> = EXAMPLE_INPUT.parse();
        assert!(puzzle.is_ok());
        assert_eq!(format!("{}", puzzle.unwrap().image), "\
            .........\n\
            .........\n\
            ..#..#...\n\
            ..#......\n\
            ..##..#..\n\
            ....#....\n\
            ....###..\n\
            .........\n\
            .........\n\
        ");
    }

    #[test]
    fn test_enhance() {
        let mut puzzle: Puzzle = EXAMPLE_INPUT.parse().unwrap();

        puzzle.enhance();
        assert_eq!(format!("{}", puzzle.image), "\
            ...........\n\
            ...........\n\
            ...##.##...\n\
            ..#..#.#...\n\
            ..##.#..#..\n\
            ..####..#..\n\
            ...#..##...\n\
            ....##..#..\n\
            .....#.#...\n\
            ...........\n\
            ...........\n\
        ");

        puzzle.enhance();
        assert_eq!(format!("{}", puzzle.image), "\
            .............\n\
            .............\n\
            .........#...\n\
            ...#..#.#....\n\
            ..#.#...###..\n\
            ..#...##.#...\n\
            ..#.....#.#..\n\
            ...#.#####...\n\
            ....#.#####..\n\
            .....##.##...\n\
            ......###....\n\
            .............\n\
            .............\n\
        ");

        assert_eq!(puzzle.get_lit_pixels(), 35);

        // Enhance until 50 times.
        for _ in 0..48 {
            puzzle.enhance();
        }

        assert_eq!(puzzle.get_lit_pixels(), 3351);
    }

    const EXAMPLE_INPUT: &str = "\
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\
        \n\
        #..#.\n\
        #....\n\
        ##..#\n\
        ..#..\n\
        ..###";
}