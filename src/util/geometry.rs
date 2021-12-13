use std::cmp::max;
use std::fmt;
use std::str::FromStr;
use crate::util::number;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<(usize, usize)> for Point {
    fn from(p: (usize, usize)) -> Self {
        Point { x: p.0, y: p.1 }
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts_result: Result<Vec<usize>, String> = s.split(",").map(|p| number::parse_usize(p)).collect();
        let parts = match parts_result {
            Ok(v) => v,
            Err(e) => return Err(e)
        };
        match parts.len() {
            2 => Ok((parts[0], parts[1]).into()),
            _ => Err(format!("Invalid str format for Point '{}', expected 'x,y'", s))
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

pub trait GetPoints {
    fn get_points(&self) -> Vec<Point>;
}

impl Line {
    fn length(&self) -> usize {
        let x1 = self.start.x;
        let x2 = self.end.x;

        if x1 > x2 { x1 - x2 } else { x2 - x1 }
    }
    fn height(&self) -> usize {
        let y1 = self.start.y;
        let y2 = self.end.y;

        if y1 > y2 { y1 - y2 } else { y2 - y1 }
    }
    fn dx(&self) -> i8 {
        let x1 = self.start.x;
        let x2 = self.end.x;

        if x1 == x2 {
            0
        } else if x2 > x1 {
            1
        } else {
            -1
        }
    }
    fn dy(&self) -> i8 {
        let y1 = self.start.y;
        let y2 = self.end.y;

        if y1 == y2 {
            0
        } else if y2 > y1 {
            1
        } else {
            -1
        }
    }
    fn x(&self, t: usize) -> usize {
        let step: isize = (t as isize) * self.dx() as isize;
        (self.start.x as isize + step) as usize
    }
    fn y(&self, t: usize) -> usize {
        let step: isize = (t as isize) * self.dy() as isize;
        (self.start.y as isize + step) as usize
    }
}

impl GetPoints for Line {
    fn get_points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        let length = self.length();
        let height = self.height();

        if length == 0 && height == 0 {
            return points;
        }

        // Given the puzzle, the lines seem to be either horizontal, vertical, or 45deg.
        // We'll panic for any other case for now.
        if length != 0 && height != 0 && length != height {
            panic!("Cannot get points for line {:?}", self);
        }

        let steps = max(length, height);
        for i in 0..steps + 1 {
            points.push((self.x(i), self.y(i)).into());
        }

        points
    }
}

#[derive(Eq, PartialEq)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<usize>>,
}

#[repr(u8)]
#[derive(Eq, PartialEq, Clone)]
pub enum Directions {
    Horizontal = 1,
    Vertical = 2,
    Diagonal = 4,
    NonDiagonal = 3,
    // Horizontal | Vertical
    All = 7, // Horizontal | Vertical | Diagonal
}

impl Directions {
    fn has(&self, value: Directions) -> bool {
        (self.clone() as u8 & value as u8) != 0
    }
}

#[allow(unused)]
impl Grid {
    pub fn get(&self, p: &Point) -> Option<usize> {
        self.cells.get(p.y).and_then(|r| r.get(p.x)).map(|x| x.clone())
    }

    pub fn get_mut(&mut self, p: &Point) -> Option<&mut usize> {
        self.cells.get_mut(p.y).and_then(|r| r.get_mut(p.x))
    }

    pub fn get_adjacent(&self, p: &Point, directions: Directions) -> Vec<usize> {
        self.get_adjacent_points(p, directions).iter().map(|p| self.get(p).unwrap()).collect()
    }

    pub fn get_adjacent_points(&self, p: &Point, directions: Directions) -> Vec<Point> {
        let mut points = vec![];

        let x = p.x;
        let y = p.y;

        let include_vertical = directions.has(Directions::Vertical);
        let include_horizontal = directions.has(Directions::Horizontal);
        let include_diagonal = directions.has(Directions::Diagonal);

        let include_top = y > 0;
        let include_bottom = y < self.height - 1;
        let include_left = x > 0;
        let include_right = x < self.width - 1;

        if include_top && include_horizontal { points.push((x, y - 1).into()) } // top
        if include_top && include_right && include_diagonal { points.push((x + 1, y - 1).into()) } // top-right
        if include_right && include_horizontal { points.push((x + 1, y).into()) } // right
        if include_bottom && include_right && include_diagonal { points.push((x + 1, y + 1).into()) } // bottom-right
        if include_bottom && include_vertical { points.push((x, y + 1).into()) } // bottom
        if include_bottom && include_left && include_diagonal { points.push((x - 1, y + 1).into()) } // bottom-left
        if include_left && include_horizontal { points.push((x - 1, y).into()) } // left
        if include_left && include_top && include_diagonal { points.push((x - 1, y - 1).into()) } // top-left

        points
    }

    pub fn points(&self) -> Vec<Point> {
        let mut points = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                points.push((x, y).into());
            }
        }

        points
    }

    pub fn values(&self) -> Vec<usize> {
        self.cells.iter().flat_map(|r| r.iter().map(|v| v.clone())).collect()
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grid")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("map", &format_args!("{}", &self))
            .finish()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines: Vec<String> = self.cells.iter().map(|l| l.iter().map(|v| format!("{}", v)).collect::<String>()).collect();
        write!(f, "{}", lines.join("\n"))
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_result: Result<Vec<Vec<usize>>, String> = s.lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| number::parse_usize(c.to_string().as_str())).collect::<Result<Vec<usize>, String>>())
            .collect();

        let cells = match parse_result {
            Ok(lines) if lines.len() == 0 => {
                return Ok(Grid { width: 0, height: 0, cells: vec![] });
            }
            Ok(lines) => lines,
            Err(e) => return Err(e)
        };

        let height = cells.len();
        let width = cells[0].len();
        if !cells.iter().all(|l| l.len() == width) {
            return Err(format!("Not all lines in input are the same width"));
        }

        Ok(Grid { width, height, cells })
    }
}

#[cfg(test)]
mod tests {
    use crate::util::geometry::{GetPoints, Line, Point, Grid, Directions};

    const fn point(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    const fn line(x1: usize, y1: usize, x2: usize, y2: usize) -> Line {
        Line { start: point(x1, y1), end: point(x2, y2) }
    }

    #[test]
    fn test_get_points() {
        assert_eq!(line(12, 0, 12, 6).get_points(), vec![point(12, 0), point(12, 1), point(12, 2), point(12, 3), point(12, 4), point(12, 5), point(12, 6)]);
        assert_eq!(line(2, 2, 4, 4).get_points(), vec![point(2, 2), point(3, 3), point(4, 4)]);
        assert_eq!(line(4, 0, 2, 0).get_points(), vec![point(4, 0), point(3, 0), point(2, 0)]);
    }

    const EXAMPLE_GRID_INPUT: &str = "\
        2199943210\n\
        3987894921\n\
        9856789892\n\
        8767896789\n\
        9899965678\
    ";

    fn get_example_grid() -> Grid {
        Grid {
            width: 10,
            height: 5,
            cells: vec![
                vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
            ],
        }
    }

    #[test]
    fn test_grid_debug() {
        assert_eq!(format!("{:?}", get_example_grid()),
                   "Grid { \
                       width: 10, \
                       height: 5, \
                       map: 2199943210\n\
                            3987894921\n\
                            9856789892\n\
                            8767896789\n\
                            9899965678 \
                   }");
    }

    #[test]
    fn test_grid_format() {
        assert_eq!(format!("{}", get_example_grid()), EXAMPLE_GRID_INPUT);
    }

    #[test]
    fn test_grid_from_str() {
        assert_eq!(EXAMPLE_GRID_INPUT.parse::<Grid>(), Ok(get_example_grid()));
    }

    #[test]
    fn test_get_adjacent() {
        let grid = get_example_grid();
        assert_eq!(grid.get_adjacent(&(0, 0).into(), Directions::NonDiagonal), vec![1, 3]);
        assert_eq!(grid.get_adjacent(&(0, 0).into(), Directions::All), vec![1, 9, 3]);
        assert_eq!(grid.get_adjacent(&(5, 0).into(), Directions::NonDiagonal), vec![3, 9, 9]);
        assert_eq!(grid.get_adjacent(&(5, 3).into(), Directions::NonDiagonal), vec![8, 6, 6, 8]);
        assert_eq!(grid.get_adjacent(&(9, 4).into(), Directions::NonDiagonal), vec![9, 7]);
    }

    #[test]
    fn test_get_adjacent_points() {
        let grid = get_example_grid();

        assert_eq!(grid.get_adjacent_points(&(0, 0).into(), Directions::NonDiagonal), vec![point(1, 0), point(0, 1)]);
        assert_eq!(grid.get_adjacent_points(&(0, 0).into(), Directions::All), vec![point(1, 0), point(1, 1), point(0, 1)]);

        assert_eq!(grid.get_adjacent_points(&(5, 3).into(), Directions::NonDiagonal),
                   vec![point(5, 2), point(6, 3), point(5, 4), point(4, 3)]);
        assert_eq!(grid.get_adjacent_points(&(5, 3).into(), Directions::All),
                   vec![point(5, 2), point(6, 2), point(6, 3), point(6, 4), point(5, 4), point(4, 4), point(4, 3), point(4, 2)]);
    }

    #[test]
    fn test_values() {
        let grid = Grid { width: 3, height: 3, cells: vec![vec![1, 2, 3], vec![9, 8, 7], vec![5, 6, 4]] };
        assert_eq!(grid.values(), vec![1, 2, 3, 9, 8, 7, 5, 6, 4]);
    }
}