use std::cmp::max;
use std::collections::HashMap;
use crate::days::Day;
use crate::util::number;

pub const DAY5: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let map = match parse_lines(input) {
        Err(e) => panic!("{}", e),
        Ok(v) => build_vent_map(&v, true)
    };

    let result = map.cells.values().filter(|c| c.value >= 2).count();
    println!("Puzzle 1 answer: {}", result);
}

fn puzzle2(input: &String) {
    let map = match parse_lines(input) {
        Err(e) => panic!("{}", e),
        Ok(v) => build_vent_map(&v, false)
    };

    let result = map.cells.values().filter(|c| c.value >= 2).count();
    println!("Puzzle 2 answer: {}", result);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Point {
    x: i128,
    y: i128,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Line {
    start: Point,
    end: Point,
}

fn parse_point(input: &str) -> Result<Point, String> {
    let parts: Vec<&str> = input.split(",").collect();
    if parts.len() != 2 { return Err(format!("Expected point x,y, but got: {}", input)); }

    let parse_result: Result<Vec<i128>, String> = parts.iter().map(|i| number::parse_i128(i)).collect();

    match parse_result {
        Ok(v) => Ok(Point { x: v[0], y: v[1] }),
        Err(e) => Err(e)
    }
}

fn parse_line(input: &str) -> Result<Line, String> {
    let parts: Vec<&str> = input.split(" -> ").collect();
    if parts.len() != 2 { return Err(format!("Expected format x1,y1 -> x2,y2, but got: {}", input)); }

    let parse_result: Result<Vec<Point>, String> = parts.iter().map(|i| parse_point(i)).collect();

    match parse_result {
        Ok(v) => Ok(Line { start: v[0], end: v[1] }),
        Err(e) => Err(e)
    }
}

fn parse_lines(input: &str) -> Result<Vec<Line>, String> {
    input.lines().filter(|l| !l.trim().is_empty()).map(|l| parse_line(l)).collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Cell {
    value: u128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct VentMap {
    cells: HashMap<Point, Cell>,
    width: usize,
    height: usize,
}

fn build_vent_map(lines: &Vec<Line>, only_horizontal_or_vertical: bool) -> VentMap {
    let mut width: usize = 0;
    let mut height: usize = 0;

    let mut map: HashMap<Point, Cell> = HashMap::new();

    for line in lines.iter().filter(|l| !only_horizontal_or_vertical || is_horizontal_or_vertical_line(l)) {
        for point in line.get_points() {
            map.insert(point, match map.get(&point) {
                Some(c) => Cell { value: c.value + 1 },
                None => Cell { value: 1 }
            });
            width = max(width, point.x as usize + 1);
            height = max(height, point.y as usize + 1);
        }
    }

    VentMap { cells: map, width, height }
}

fn is_horizontal_or_vertical_line(line: &Line) -> bool {
    line.start.x == line.end.x || line.start.y == line.end.y
}

trait GetPoints {
    fn get_points(&self) -> Vec<Point>;
}

impl GetPoints for Line {
    fn get_points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        let length = self.end.x - self.start.x;
        let height = self.end.y - self.start.y;

        if length == 0 && height == 0 {
            return points;
        }

        // Given the puzzle, the lines seem to be either horizontal, vertical, or 45deg.
        // We'll panic for any other case for now.
        if length != 0 && height != 0 && length.abs() != height.abs() {
            panic!("Cannot get points for line {:?}", self);
        }

        let dx: i128 = if length == 0 { 0 } else if length > 0 { 1 } else { -1 };
        let dy: i128 = if height == 0 { 0 } else if height > 0 { 1 } else { -1 };
        let steps = max(length.abs(), height.abs());

        for i in 0..steps + 1 {
            let x = self.start.x + i * dx;
            let y = self.start.y + i * dy;
            points.push(Point { x, y });
        }

        points
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::days::day05::{build_vent_map, Cell, GetPoints, Line, parse_line, parse_lines, parse_point, Point, VentMap};

    #[test]
    fn test_parse_point() {
        assert_eq!(parse_point("12,0"), Ok(Point { x: 12, y: 0 }));
        assert_eq!(parse_point("1337,42"), Ok(Point { x: 1337, y: 42 }));
        assert!(parse_point("12,a").is_err());
        assert!(parse_point("12").is_err());
        assert!(parse_point("12,1,3").is_err());
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("12,0 -> 12,6"), Ok(Line { start: Point { x: 12, y: 0 }, end: Point { x: 12, y: 6 } }));
        assert!(parse_line("12,0 -> 12,6 -> 12,7").is_err());
        assert!(parse_line("12,0 -> x,y").is_err());
        assert!(parse_line("12,0 -> 12").is_err());
        assert!(parse_line("12,0 <- 12,5").is_err());
    }

    const fn point(x: i128, y: i128) -> Point {
        Point { x, y }
    }

    const fn line(x1: i128, y1: i128, x2: i128, y2: i128) -> Line {
        Line { start: point(x1, y1), end: point(x2, y2) }
    }

    #[test]
    fn test_get_points() {
        assert_eq!(line(12, 0, 12, 6).get_points(), vec![point(12, 0), point(12, 1), point(12, 2), point(12, 3), point(12, 4), point(12, 5), point(12, 6)]);
        assert_eq!(line(2, 2, 4, 4).get_points(), vec![point(2, 2), point(3, 3), point(4, 4)]);
        assert_eq!(line(4, 0, 2, 0).get_points(), vec![point(4, 0), point(3, 0), point(2, 0)]);
    }

    const EXAMPLE_INPUT: &str = "\
        0,9 -> 5,9\n\
        8,0 -> 0,8\n\
        9,4 -> 3,4\n\
        2,2 -> 2,1\n\
        7,0 -> 7,4\n\
        6,4 -> 2,0\n\
        0,9 -> 2,9\n\
        3,4 -> 1,4\n\
        0,0 -> 8,8\n\
        5,5 -> 8,2
    ";

    const EXAMPLE_LINES: [Line; 10] = [
        line(0, 9, 5, 9),
        line(8, 0, 0, 8),
        line(9, 4, 3, 4),
        line(2, 2, 2, 1),
        line(7, 0, 7, 4),
        line(6, 4, 2, 0),
        line(0, 9, 2, 9),
        line(3, 4, 1, 4),
        line(0, 0, 8, 8),
        line(5, 5, 8, 2)
    ];

    #[test]
    fn test_parse_input() {
        let result = parse_lines(EXAMPLE_INPUT);

        assert_eq!(result, Ok(EXAMPLE_LINES.to_vec()));
    }

    #[test]
    fn test_build_vent_map_puzzle1() {
        let result = build_vent_map(&EXAMPLE_LINES.to_vec(), true);

        assert_eq!(result, VentMap {
            cells: HashMap::from([
                (point(7, 0), Cell { value: 1 }),
                (point(2, 1), Cell { value: 1 }),
                (point(7, 1), Cell { value: 1 }),
                (point(2, 2), Cell { value: 1 }),
                (point(7, 2), Cell { value: 1 }),
                (point(7, 3), Cell { value: 1 }),
                (point(1, 4), Cell { value: 1 }),
                (point(2, 4), Cell { value: 1 }),
                (point(3, 4), Cell { value: 2 }),
                (point(4, 4), Cell { value: 1 }),
                (point(5, 4), Cell { value: 1 }),
                (point(6, 4), Cell { value: 1 }),
                (point(7, 4), Cell { value: 2 }),
                (point(8, 4), Cell { value: 1 }),
                (point(9, 4), Cell { value: 1 }),
                (point(0, 9), Cell { value: 2 }),
                (point(1, 9), Cell { value: 2 }),
                (point(2, 9), Cell { value: 2 }),
                (point(3, 9), Cell { value: 1 }),
                (point(4, 9), Cell { value: 1 }),
                (point(5, 9), Cell { value: 1 }),
            ]),
            width: 10,
            height: 10,
        });
    }

    #[test]
    fn test_build_vent_map_puzzle2() {
        let result = build_vent_map(&EXAMPLE_LINES.to_vec(), false);

        assert_eq!(result, VentMap {
            cells: HashMap::from([
                (point(0, 0), Cell { value: 1 }),
                (point(2, 0), Cell { value: 1 }),
                (point(7, 0), Cell { value: 1 }),
                (point(8, 0), Cell { value: 1 }),
                (point(1, 1), Cell { value: 1 }),
                (point(2, 1), Cell { value: 1 }),
                (point(3, 1), Cell { value: 1 }),
                (point(7, 1), Cell { value: 2 }),
                (point(2, 2), Cell { value: 2 }),
                (point(4, 2), Cell { value: 1 }),
                (point(6, 2), Cell { value: 1 }),
                (point(7, 2), Cell { value: 1 }),
                (point(8, 2), Cell { value: 1 }),
                (point(3, 3), Cell { value: 1 }),
                (point(5, 3), Cell { value: 2 }),
                (point(7, 3), Cell { value: 2 }),
                (point(1, 4), Cell { value: 1 }),
                (point(2, 4), Cell { value: 1 }),
                (point(3, 4), Cell { value: 2 }),
                (point(4, 4), Cell { value: 3 }),
                (point(5, 4), Cell { value: 1 }),
                (point(6, 4), Cell { value: 3 }),
                (point(7, 4), Cell { value: 2 }),
                (point(8, 4), Cell { value: 1 }),
                (point(9, 4), Cell { value: 1 }),
                (point(3, 5), Cell { value: 1 }),
                (point(5, 5), Cell { value: 2 }),
                (point(2, 6), Cell { value: 1 }),
                (point(6, 6), Cell { value: 1 }),
                (point(1, 7), Cell { value: 1 }),
                (point(7, 7), Cell { value: 1 }),
                (point(0, 8), Cell { value: 1 }),
                (point(8, 8), Cell { value: 1 }),
                (point(0, 9), Cell { value: 2 }),
                (point(1, 9), Cell { value: 2 }),
                (point(2, 9), Cell { value: 2 }),
                (point(3, 9), Cell { value: 1 }),
                (point(4, 9), Cell { value: 1 }),
                (point(5, 9), Cell { value: 1 }),
            ]),
            width: 10,
            height: 10,
        });
    }
}